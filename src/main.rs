
use log::{info, error, LevelFilter};
use structopt::StructOpt;
use simplelog::{SimpleLogger, Config as LogConfig};

use s3::{
    creds::Credentials,
    region::Region,
    bucket::Bucket,
};

use glob::glob as globber;

#[derive(Clone, PartialEq, Debug, StructOpt)]
pub struct Options {
    /// Access key for bucket
    #[structopt(long, env)]
    pub access_key: String,
    /// Secret key for bucket
    #[structopt(long, env)]
    pub secret_key: String,

    /// Bucket name
    #[structopt(long, env="S3_BUCKET")]
    pub bucket: String,

    /// Bucket region (eg. s3-ap-northeast-1)
    #[structopt(long, env="S3_REGION")]
    pub region: String,

    /// Bucket endpoint (eg. amazonaws.com)
    #[structopt(long, env="S3_ENDPOINT")]
    pub endpoint: String,

    #[structopt(subcommand)]
    command: Command,

    #[structopt(long, default_value="info")]
    pub log_level: LevelFilter,
}
#[derive(Clone, PartialEq, Debug, StructOpt)]
pub enum Command {
    /// Show items in bucket
    List{
        #[structopt(long, default_value="")]
        prefix: String
    },
    /// Upload an item to the bucket
    Upload{
        /// Name of object in bucket
        name: String,
        /// File to upload
        file: String,
    },
    /// Upload files from a directory
    UploadDir{
        /// Prefix for files in bucket
        #[structopt(long, default_value="")]
        prefix: String,
        /// Glob for matching files to upload
        glob: String,
    },
    /// Download an item from the bucket
    Download{
        /// Name of object in bucket
        name: String,
        /// File to download
        file: String,
    },
    /// Delete an item from the bucket
    Delete{
        /// Name of object in bucket
        name: String,
    }
}

#[async_std::main]
async fn main() -> Result<(), anyhow::Error> {

    // Parse arguments
    let opts = Options::from_args();

    // Initialise logging
    let _ = SimpleLogger::init(opts.log_level, LogConfig::default());

    // Setup credentials
    let creds = Credentials::new(Some(&opts.access_key), Some(&opts.secret_key), None, None, None)?;
    let region = Region::Custom{ region: opts.region, endpoint: opts.endpoint };

    // Setup storage
    let bucket = Bucket::new(&opts.bucket, region, creds)?;

    // Execute command
    match &opts.command {
        Command::List{ prefix } => {
            for list in bucket.list(prefix.to_string(), None).await? {
                //TODO: fix this to a propper print
                println!("{:?}", list);
            }
        },
        Command::Upload{ name, file } => {
            let files: Vec<_> = globber(file)?.filter_map(|v| v.ok() ).collect();
            
            if files.len() == 0 {
                return Err(anyhow::anyhow!("No matching file found"));
            } else if files.len() > 1 {
                return Err(anyhow::anyhow!("Too many matching files"));
            }
            let f = &files[0];

            info!("Loading file '{}'", f.to_str().unwrap());
            let data = std::fs::read(f)?;

            info!("Uploading object: '{}'", name);
            let (_, code) = bucket.put_object(name, &data).await?;
            if code != 200 {
                return Err(anyhow::anyhow!("Error uploading object: {}", code));
            }

            info!("Upload complete");
        },
        Command::UploadDir{ prefix, glob } => {
            let mut count = 0usize;
            for e in globber(glob)? {
                // For each viable path
                let p = match e {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Error reading file: {:?}", e);
                        continue;
                    }
                };

                // Grab the file name
                let f = match p.file_name() {
                    Some(n) => n.to_str().unwrap(),
                    None => continue,
                };

                let n = format!("{}{}", prefix, f);

                info!("Uploading {} as {}", p.to_str().unwrap(), f);

                // Upload the file
                let data = std::fs::read(p)?;
                let (_, code) = bucket.put_object(n, &data).await?;
                if code != 200 {
                    return Err(anyhow::anyhow!("Error uploading object: {}", code));
                }

                count += 1;
            }

            info!("Uploaded {} files", count);

        }
        Command::Download{ name, file } => {
            info!("Fetching object: '{}'", name);
            let (data, code) = bucket.get_object(name).await?;
            if code != 200 {
                return Err(anyhow::anyhow!("Error fetching object: {}", code));
            }

            info!("Writing file: '{}'", file);
            std::fs::write(file, data)?;

            info!("File write done");
        },
        Command::Delete{ name } => {
            let (_, code) = bucket.delete_object( name ).await?;
            if code != 204 {
                return Err(anyhow::anyhow!("Error deleting object: {}", code));
            }
        }
    }

    Ok(())
}
