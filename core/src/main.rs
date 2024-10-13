use converter::ConversionOption;
use utility::string;

mod converter;
mod probe;
mod utility;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let input = string("demo.mkv");
    let output = string("demo.mp4");

    let opts = ConversionOption {
        input,
        output,
        ..Default::default()
    };
    
    converter::run(opts).await?;
    Ok(())
}
