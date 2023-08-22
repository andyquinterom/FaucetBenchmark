use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// The target URL to stress. #[arg(short, long)]
    target: Url,

    /// Number of times to repeat the requests.
    #[arg(short, long)]
    iterations: usize,

    /// Simultaenous requests.
    #[arg(short, long)]
    concurrency: usize,

}

impl Args {
    pub fn get_url(&self, path: &str, weight: usize) -> Url {
        let mut url = self.target.clone();
        url.set_path(path);
        url.query_pairs_mut()
            .append_pair("weight", &weight.to_string());
        url
    }
    pub fn get_url_string(&self) -> String {
        self.target.to_string()
    }
    pub fn get_iterations(&self) -> usize {
        self.iterations
    }
    pub fn get_concurrency(&self) -> usize {
        self.concurrency
    }
}
