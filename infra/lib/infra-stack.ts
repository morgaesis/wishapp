import * as cdk from "aws-cdk-lib";
import * as iam from "aws-cdk-lib/aws-iam";
import { Construct } from "constructs";

/**
 * Properties required for configuring the Wishapp infrastructure stack
 */
interface WishappStackProps extends cdk.StackProps {
  /**
   * GitHub organization name where the repository is hosted
   */
  readonly githubOrg: string;
  /**
   * Name of the GitHub repository
   */
  readonly githubRepo: string;
}

/**
 * Main infrastructure stack for the Wishapp application
 *
 * This stack creates the necessary AWS resources for GitHub Actions deployments:
 * - OIDC provider configuration for secure GitHub Actions authentication
 * - IAM role with appropriate permissions for deployments
 */
export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: WishappStackProps) {
    super(scope, id, props);

    // Create OIDC (OpenID Connect) provider for GitHub Actions
    // This enables secure authentication between GitHub Actions and AWS
    // without storing long-lived credentials
    const oidcProvider = iam.OpenIdConnectProvider.fromOpenIdConnectProviderArn(
      this,
      "GitHubOIDCProvider",
      `arn:aws:iam::${this.account}:oidc-provider/token.actions.githubusercontent.com`
    );

    // Create deployment role with GitHub OIDC trust relationship
    // This role can only be assumed by GitHub Actions workflows running on:
    // 1. Pull requests in the specified repository
    // 2. Push events to the main branch
    const deployRole = new iam.Role(this, "GitHubDeployRole", {
      assumedBy: new iam.FederatedPrincipal(
        oidcProvider.openIdConnectProviderArn,
        {
          StringLike: {
            [`token.actions.githubusercontent.com:sub`]: [
              `repo:${props.githubOrg}/${props.githubRepo}:pull_request`,
              `repo:${props.githubOrg}/${props.githubRepo}:ref:refs/heads/main`,
            ],
          },
        },
        "sts:AssumeRoleWithWebIdentity"
      ),
      description: "Role for GitHub Actions to deploy WishApp",
      maxSessionDuration: cdk.Duration.hours(1),
    });

    // Configure IAM permissions following the principle of least privilege
    // S3 Permissions - For managing static assets and user-uploaded content
    deployRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "s3:PutObject",
          "s3:GetObject",
          "s3:ListBucket",
          "s3:DeleteObject",
        ],
        resources: [
          `arn:aws:s3:::wishapp-assets-${this.account}`,
          `arn:aws:s3:::wishapp-assets-${this.account}/*`,
        ],
      })
    );

    // DynamoDB Permissions - For user data and wish lists
    deployRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "dynamodb:PutItem",
          "dynamodb:GetItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:Query",
          "dynamodb:Scan",
        ],
        resources: [
          `arn:aws:dynamodb:${this.region}:${this.account}:table/wishapp-*`,
        ],
      })
    );

    // CloudFront Permissions - For CDN management
    deployRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "cloudfront:CreateInvalidation",
          "cloudfront:GetDistribution",
          "cloudfront:UpdateDistribution",
        ],
        resources: [`arn:aws:cloudfront::${this.account}:distribution/*`],
      })
    );

    // Lambda Permissions - For serverless functions
    deployRole.addToPolicy(
      new iam.PolicyStatement({
        actions: [
          "lambda:UpdateFunctionCode",
          "lambda:UpdateFunctionConfiguration",
          "lambda:GetFunction",
          "lambda:InvokeFunction",
        ],
        resources: [
          `arn:aws:lambda:${this.region}:${this.account}:function:wishapp-*`,
        ],
      })
    );
  }
}
