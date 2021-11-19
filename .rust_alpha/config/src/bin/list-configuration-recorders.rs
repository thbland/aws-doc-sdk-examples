/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_config::{Client, Error, Region, PKG_VERSION};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Lists your recorders.
// snippet-start:[config.rust.list-configuration-recorders]
async fn show_recorders(client: &Client) -> Result<(), Error> {
    let resp = client.describe_configuration_recorders().send().await?;

    let recorders = resp.configuration_recorders().unwrap_or_default();

    if recorders.is_empty() {
        println!("You have no configuration recorders")
    } else {
        for recorder in recorders {
            println!("Recorder: {}", recorder.name().as_deref().unwrap_or_default());
        }
    }

    println!();

    Ok(())
}
// snippet-end:[config.rust.list-configuration-recorders]

/// Lists the AWS Config configuration recorders in the Region.
///
/// # Arguments
///
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt { region, verbose } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("Config client version: {}", PKG_VERSION);
        println!(
            "Region:                {}",
            region_provider.region().await.unwrap().as_ref()
        );

        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    show_recorders(&client).await
}
