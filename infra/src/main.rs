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

        // TODO: Replace AdministratorAccess with specific permissions
        // Current permissions are overly permissive. Should be replaced with:
        // - S3 access for static assets
        // - DynamoDB access for application data
        // - CloudFront permissions for CDN management
        // - Lambda permissions for serverless functions
        deploy_role.add_managed_policy(ManagedPolicy::from_aws_managed_policy_name("AdministratorAccess"));

        Self { stack }
    }
}

/// Entry point for the CDK application
/// 
/// Creates and synthesizes the Wishapp infrastructure stack.
/// TODO: Replace hardcoded values with environment variables or command-line arguments
fn main() {
    let app = App::new();
    WishappStack::new(
        &app,
        "WishappStack",
        // These values should be configurable through environment variables
        WishappStackProps::new("morgaesis", "wishapp")
    );
    app.synth();
}