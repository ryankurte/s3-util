
use log::{info, LevelFilter};
use structopt::StructOpt;
use simplelog::{SimpleLogger, Config as LogConfig};

use s3::{
    creds::Credentials,
    region::Region,
    bucket::Bucket,
};

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
            info!("Loading file '{}'", file);
            let data = std::fs::read(file)?;

            info!("Uploading object: '{}'", name);
            let (_, code) = bucket.put_object(name, &data).await?;
            if code != 200 {
                return Err(anyhow::anyhow!("Error uploading object: {}", code));
            }

            info!("Upload complete");
        },
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
