import * as cdk from "aws-cdk-lib";
import * as iam from "aws-cdk-lib/aws-iam";
import * as sqs from "aws-cdk-lib/aws-sqs";
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

    // Validate required context
    const githubRepo = this.node.tryGetContext('githubRepo');
    if (!githubRepo) {
      throw new Error('githubRepo context variable is required');
    }

    // Environment configuration
    const prNumber = this.node.tryGetContext('prNumber');
    const isPrEnv = !!prNumber;
    const envPrefix = isPrEnv ? `pr-${prNumber}-` : '';
    const stackSuffix = isPrEnv ? `-pr-${prNumber}` : '-prod';
    
    // Tag all resources for cost tracking and isolation
    cdk.Tags.of(this).add('StackType', `wishapp${stackSuffix}`);
    cdk.Tags.of(this).add('Environment', isPrEnv ? 'pr' : 'prod');
    cdk.Tags.of(this).add('GitHubRepo', githubRepo);

    // OIDC Provider (must exist)
    const oidcProvider = iam.OpenIdConnectProvider.fromOpenIdConnectProviderArn(
      this,
      'GitHubOIDCProvider',
      `arn:aws:iam::${this.account}:oidc-provider/token.actions.githubusercontent.com`
    );

    // GitHub Actions Role with conditional permissions
    const githubActionsRole = new iam.Role(this, 'GitHubActionsRole', {
      assumedBy: new iam.WebIdentityPrincipal(oidcProvider.openIdConnectProviderArn, {
        'StringLike': {
          'token.actions.githubusercontent.com:sub': [
            `repo:${githubRepo}:pull_request`,
            `repo:${githubRepo}:ref:refs/heads/main`
          ]
        }
      }),
      roleName: `github-actions${stackSuffix}`,
      maxSessionDuration: cdk.Duration.hours(1),
      inlinePolicies: {
        deploymentAccess: new iam.PolicyDocument({
          statements: [
            new iam.PolicyStatement({
              actions: ['cloudformation:*', 's3:*', 'iam:*'],
              resources: ['*'],
              conditions: {
                'ArnEquals': {
                  'aws:ResourceTag/StackType': `wishapp${stackSuffix}`
                }
              }
            })
          ]
        })
      }
    });

    // Use existing stackSuffix variable declared earlier
    
    // Deployment role with conditional trust
    const deployRole = new iam.Role(this, `GitHubDeployRole${stackSuffix}`, {
      assumedBy: new iam.WebIdentityPrincipal(oidcProvider.openIdConnectProviderArn, {
        StringEquals: {
          'token.actions.githubusercontent.com:aud': 'sts.amazonaws.com'
        },
        StringLike: {
          'token.actions.githubusercontent.com:sub': [
            `repo:${props.githubOrg}/${props.githubRepo}:pull_request`,
            `repo:${props.githubOrg}/${props.githubRepo}:ref:refs/heads/main`
          ]
        }
      }),
      description: isPrEnv 
        ? `Temporary deployment role for PR #${prNumber} (auto-cleanup)`
        : 'Production deployment role',
      maxSessionDuration: cdk.Duration.hours(1)
    });

    // Add required permissions to the deployment role
    deployRole.addToPolicy(new iam.PolicyStatement({
      actions: [
        'sts:AssumeRole',
        'cloudformation:*',
        'iam:PassRole'
      ],
      resources: ['*'],
      conditions: isPrEnv 
        ? {
            'StringEquals': {
              'aws:RequestedRegion': [this.region],
              'aws:PrincipalTag/PR': [prNumber]
            }
          }
        : undefined
    }));

    // Create application resources with PR-aware naming
    const resourcePrefix = isPrEnv ? `pr-${prNumber}-` : '';

    // Create SQS queue with PR-aware naming
    const queue = new sqs.Queue(this, 'WishQueue', {
      queueName: `${resourcePrefix}wish-queue`,
      visibilityTimeout: cdk.Duration.seconds(300)
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

    // Example SQS Queue - For test purposes
    new sqs.Queue(this, "ExampleQueue", {
      visibilityTimeout: cdk.Duration.seconds(300),
    });

    // Export the OIDC role ARN for GitHub Actions
    new cdk.CfnOutput(this, "GitHubOidcRoleArn", {
      value: deployRole.roleArn,
      exportName: "GitHubOidcRoleArn",
    });
  }
}
