use crate::subs::runnable::RunnableSubcommand;
use anyhow::Result;
use clap::Parser;
use solhat::calibrationframe::CalibrationImage;
use solhat::calibrationframe::ComputeMethod;

pb_create_spinner!();

#[derive(Parser)]
#[command(author, version, about = "Create a calibration as the average of pixels", long_about = None)]
pub struct Mean {
    #[clap(long, short, help = "Input ser file")]
    input_files: String,

    #[clap(long, short, help = "Output image")]
    output: String,
}

#[async_trait::async_trait]
impl RunnableSubcommand for Mean {
    async fn run(&self) -> Result<()> {
        pb_set_print!();
        let calimage = CalibrationImage::new_from_file(&self.input_files, ComputeMethod::Mean)?;
        calimage.image.unwrap().save(&self.output)?;
        pb_done!();
        Ok(())
    }
}
