# News Aggregator Lambda Function Using Rust

This project is a serverless news aggregator Rust API using AWS Lambda and the News API.

## Example Query and Output

Query:
```
https://your-lambda-function-url.amazonaws.com/?query=tesla&news_api_key=your_news_api_key_here
```

Output:
```json
{
    "req_id": "3389cab0-c041-4c0c-99bc-1981a2288b4c",
    "response": {
        "articles": [
            {
                "headline": "I Own a Chevy Bolt, and Superchargers Are a Total Game Changer",
                "publishedAt": "2024-10-04T22:04:17Z",
                "publisher": "Wired"
            },
            {
                "headline": "Tesla Semi fire required 50,000 gallons of water to extinguish",
                "publishedAt": "2024-09-13T12:00:06Z",
                "publisher": "Yahoo Entertainment"
            },
            {
                "headline": "Tesla Is Ready to Roll Out the Cybercab, Its Answer to Robotaxis",
                "publishedAt": "2024-10-09T12:30:00Z",
                "publisher": "Wired"
            },
            // ... more articles ...
        ]
    }
}
```

## Setup

1. **AWS Account**: Ensure you have an AWS account and the AWS CLI configured. If not use aws configure and provide an IAM user's AWS access key ids.

2. **News API Key**: 
   - Sign up at [newsapi.org](https://newsapi.org) to get an API key.

3. **Lambda Function**:
   - Use this tutorial to set up your lambda function utilising Rust:
   [Deploy Rust Lambda functions with .zip file archives](https://docs.aws.amazon.com/lambda/latest/dg/rust-package.html)

4. **IAM Permissions**: Add SSM read permissions to your Lambda function's role.

5. **Build and Deploy**:
   - Build the project: `cargo lambda build --release --target x86_64-unknown-linux-gnu`
   - Deploy: `cargo lambda deploy news-aggregator`

6. **Enable Function URL**:
   - In the AWS Lambda console, go to your function and select the "Configuration" tab.
   - In the left sidebar, click on "Function URL".
   - Click "Create function URL" and configure it as follows:
     - Auth type: NONE
     - Configure cross-origin resource sharing (CORS): Enabled
     - Allow origin: * (or specify your allowed origins)
     - Allow headers: *
     - Allow methods: GET, POST
     - Allow credentials: Yes
   - Click "Save" to create the function URL.

   This configuration allows public access to your function URL without IAM authentication and enables full CORS access.

## Usage

To use the Lambda function, you can invoke it using a curl command with query parameters:

```bash
curl "https://your-lambda-function-url.amazonaws.com/?query=technology&news_api_key=your_news_api_key_here"
```

Replace `your-lambda-function-url` with your actual Lambda function URL, and `your_news_api_key_here` with your News API key.

This will return news articles related to the query "technology" from the News API.

## Development

- Main logic is in `src/main.rs`.
- Use `cargo lambda watch` for local development.
- Run `cargo test` for unit tests.