use aws_cdk_lib::{App, Stack, StackProps};
use aws_cdk_lib::aws_iam::{Role, ServicePrincipal, PolicyStatement, FederatedPrincipal, ManagedPolicy};
use aws_cdk_lib::aws_iam::OidcProvider;
use constructs::Construct;

// SECURITY WARNING: AdministratorAccess provides excessive privileges
// Consider granular permissions instead of full admin access
// TODO: Replace with least-privilege policy for specific deployment needs

struct WishappStackProps {
    github_org: String,
    github_repo: String,
}

impl WishappStackProps {
    fn new(github_org: &str, github_repo: &str) -> Self {
        WishappStackProps {
            github_org: github_org.to_string(),
            github_repo: github_repo.to_string(),
        }
    }
}

struct WishappStack {
    stack: Stack,
}

impl WishappStack {
    fn new(scope: &Construct, id: &str, props: WishappStackProps) -> Self {
        let stack = Stack::new(scope, id, &StackProps::default());
        
        // Create OIDC provider
        let oidc_provider = OidcProvider::from_open_id_connect_provider_arn(
            &stack,
            "GitHubOIDCProvider",
            &format!("arn:aws:iam::{}:oidc-provider/token.actions.githubusercontent.com", stack.account())
        );

        // Create deployment role
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
                max_session_duration: Some(std::time::Duration::from_secs(3600)),
                ..Default::default()
            }
        );

        // Add permissions
        deploy_role.add_managed_policy(ManagedPolicy::from_aws_managed_policy_name("AdministratorAccess"));

        Self { stack }
    }
}

fn main() {
    let app = App::new();
    WishappStack::new(
        &app,
        "WishappStack",
        WishappStackProps::new("morgaesis", "wishapp")
    );
    app.synth();
}