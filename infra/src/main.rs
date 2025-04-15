use aws_cdk_lib::{App, Stack, StackProps};
use aws_cdk_lib::aws_iam::{Role, ServicePrincipal, PolicyStatement, FederatedPrincipal, ManagedPolicy};
use aws_cdk_lib::aws_iam::OidcProvider;
use constructs::Construct;

// SECURITY WARNING: AdministratorAccess provides excessive privileges
// Consider granular permissions instead of full admin access
// TODO: Replace with least-privilege policy for specific deployment needs

/// Properties required for configuring the Wishapp infrastructure stack
struct WishappStackProps {
    /// GitHub organization name where the repository is hosted
    github_org: String,
    /// Name of the GitHub repository
    github_repo: String,
}

impl WishappStackProps {
    /// Creates a new WishappStackProps instance
    /// 
    /// # Arguments
    /// * `github_org` - GitHub organization name (e.g., "morgaesis")
    /// * `github_repo` - GitHub repository name (e.g., "wishapp")
    /// 
    /// # Note
    /// These values should ideally be configured through environment variables
    /// or configuration files rather than being hardcoded in main()
    fn new(github_org: &str, github_repo: &str) -> Self {
        WishappStackProps {
            github_org: github_org.to_string(),
            github_repo: github_repo.to_string(),
        }
    }
}

/// Main infrastructure stack for the Wishapp application
/// 
/// This stack creates the necessary AWS resources for GitHub Actions deployments:
/// - OIDC provider configuration for secure GitHub Actions authentication
/// - IAM role with appropriate permissions for deployments
struct WishappStack {
    stack: Stack,
}

impl WishappStack {
    /// Creates a new WishappStack instance with GitHub Actions deployment configuration
    /// 
    /// # Arguments
    /// * `scope` - The CDK app scope
    /// * `id` - Unique identifier for this stack
    /// * `props` - Configuration properties for the stack
    fn new(scope: &Construct, id: &str, props: WishappStackProps) -> Self {
        // Initialize the CDK stack with default properties
        let stack = Stack::new(scope, id, &StackProps::default());
        
        // Create OIDC (OpenID Connect) provider for GitHub Actions
        // This enables secure authentication between GitHub Actions and AWS
        // without storing long-lived credentials
        let oidc_provider = OidcProvider::from_open_id_connect_provider_arn(
            &stack,
            "GitHubOIDCProvider",
            &format!("arn:aws:iam::{}:oidc-provider/token.actions.githubusercontent.com", stack.account())
        );

        // Create deployment role with GitHub OIDC trust relationship
        // This role can only be assumed by GitHub Actions workflows running on:
        // 1. Pull requests in the specified repository
        // 2. Push events to the main branch
        let deploy_role = Role::new(
            &stack,
            "GitHubDeployRole",
            &aws_cdk_lib::aws_iam::RoleProps {
                assumed_by: FederatedPrincipal::new(
                    oidc_provider.open_id_connect_provider_arn(),
                    hash_map! {
                        "StringLike".to_string() => vec![
                            format!("token.actions.githubusercontent.com:sub:repo:{}/{}:pull_request", props.github_org, props.github_repo),
                            format!("token.actions.githubusercontent.com:sub:repo:{}/{}:ref:refs/heads/main", props.github_org, props.github_repo)
                        ]
                    },
                    "sts:AssumeRoleWithWebIdentity".to_string()
                ),
                description: Some("Role for GitHub Actions to deploy WishApp".to_string()),
                // Set maximum session duration to 1 hour (3600 seconds)
                // This limits the time window during which temporary credentials are valid
                max_session_duration: Some(std::time::Duration::from_secs(3600)),
                ..Default::default()
            }
        );

        // Add specific permissions for each service
        deploy_role.add_to_policy(PolicyStatement::new()
            .add_actions(vec![
                // S3 permissions for static assets
                "s3:PutObject",
                "s3:GetObject",
                "s3:ListBucket",
                "s3:DeleteObject",
            ])
            .add_resources(vec![
                format!("arn:aws:s3:::wishapp-assets-{}", stack.account()),
                format!("arn:aws:s3:::wishapp-assets-{}/*", stack.account()),
            ]));

        deploy_role.add_to_policy(PolicyStatement::new()
            .add_actions(vec![
                // DynamoDB permissions
                "dynamodb:PutItem",
                "dynamodb:GetItem",
                "dynamodb:UpdateItem",
                "dynamodb:DeleteItem",
                "dynamodb:Query",
                "dynamodb:Scan",
            ])
            .add_resources(vec![
                format!("arn:aws:dynamodb:{}:{}:table/wishapp-*", 
                    stack.region(), stack.account()),
            ]));

        deploy_role.add_to_policy(PolicyStatement::new()
            .add_actions(vec![
                // CloudFront permissions
                "cloudfront:CreateInvalidation",
                "cloudfront:GetDistribution",
                "cloudfront:UpdateDistribution",
            ])
            .add_resources(vec![
                format!("arn:aws:cloudfront::{}:distribution/*", stack.account()),
            ]));

        deploy_role.add_to_policy(PolicyStatement::new()
            .add_actions(vec![
                // Lambda permissions
                "lambda:UpdateFunctionCode",
                "lambda:UpdateFunctionConfiguration",
                "lambda:GetFunction",
                "lambda:InvokeFunction",
            ])
            .add_resources(vec![
                format!("arn:aws:lambda:{}:{}:function:wishapp-*",
                    stack.region(), stack.account()),
            ]));

        Self { stack }
    }
}

/// Entry point for the CDK application
/// 
/// Creates and synthesizes the Wishapp infrastructure stack.
/// TODO: Replace hardcoded values with environment variables or command-line arguments
/// Initialize and validate environment configuration
fn get_config() -> Result<(String, String), Box<dyn std::error::Error>> {
    let github_org = std::env::var("GITHUB_ORG")
        .map_err(|_| "GITHUB_ORG environment variable is required")?;
    let github_repo = std::env::var("GITHUB_REPO")
        .map_err(|_| "GITHUB_REPO environment variable is required")?;
    
    // Validate organization name format
    if !github_org.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("GITHUB_ORG must contain only alphanumeric characters and hyphens".into());
    }
    
    // Validate repository name format
    if !github_repo.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("GITHUB_REPO must contain only alphanumeric characters and hyphens".into());
    }
    
    Ok((github_org, github_repo))
}

fn main() {
    let app = App::new();
    
    // Get and validate configuration
    let (github_org, github_repo) = match get_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };
    
    WishappStack::new(
        &app,
        "WishappStack",
        WishappStackProps::new(&github_org, &github_repo)
    );
    app.synth();
}