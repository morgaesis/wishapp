"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.InfraStack = void 0;
const cdk = require("aws-cdk-lib");
const iam = require("aws-cdk-lib/aws-iam");
/**
 * Main infrastructure stack for the Wishapp application
 *
 * This stack creates the necessary AWS resources for GitHub Actions deployments:
 * - OIDC provider configuration for secure GitHub Actions authentication
 * - IAM role with appropriate permissions for deployments
 */
class InfraStack extends cdk.Stack {
    constructor(scope, id, props) {
        super(scope, id, props);
        // Create OIDC (OpenID Connect) provider for GitHub Actions
        // This enables secure authentication between GitHub Actions and AWS
        // without storing long-lived credentials
        const oidcProvider = iam.OpenIdConnectProvider.fromOpenIdConnectProviderArn(this, 'GitHubOIDCProvider', `arn:aws:iam::${this.account}:oidc-provider/token.actions.githubusercontent.com`);
        // Create deployment role with GitHub OIDC trust relationship
        // This role can only be assumed by GitHub Actions workflows running on:
        // 1. Pull requests in the specified repository
        // 2. Push events to the main branch
        const deployRole = new iam.Role(this, 'GitHubDeployRole', {
            assumedBy: new iam.FederatedPrincipal(oidcProvider.openIdConnectProviderArn, {
                StringLike: {
                    [`token.actions.githubusercontent.com:sub`]: [
                        `repo:${props.githubOrg}/${props.githubRepo}:pull_request`,
                        `repo:${props.githubOrg}/${props.githubRepo}:ref:refs/heads/main`
                    ]
                }
            }, 'sts:AssumeRoleWithWebIdentity'),
            description: 'Role for GitHub Actions to deploy WishApp',
            maxSessionDuration: cdk.Duration.hours(1)
        });
        // Configure IAM permissions following the principle of least privilege
        // S3 Permissions - For managing static assets and user-uploaded content
        deployRole.addToPolicy(new iam.PolicyStatement({
            actions: [
                's3:PutObject',
                's3:GetObject',
                's3:ListBucket',
                's3:DeleteObject'
            ],
            resources: [
                `arn:aws:s3:::wishapp-assets-${this.account}`,
                `arn:aws:s3:::wishapp-assets-${this.account}/*`
            ]
        }));
        // DynamoDB Permissions - For user data and wish lists
        deployRole.addToPolicy(new iam.PolicyStatement({
            actions: [
                'dynamodb:PutItem',
                'dynamodb:GetItem',
                'dynamodb:UpdateItem',
                'dynamodb:DeleteItem',
                'dynamodb:Query',
                'dynamodb:Scan'
            ],
            resources: [
                `arn:aws:dynamodb:${this.region}:${this.account}:table/wishapp-*`
            ]
        }));
        // CloudFront Permissions - For CDN management
        deployRole.addToPolicy(new iam.PolicyStatement({
            actions: [
                'cloudfront:CreateInvalidation',
                'cloudfront:GetDistribution',
                'cloudfront:UpdateDistribution'
            ],
            resources: [
                `arn:aws:cloudfront::${this.account}:distribution/*`
            ]
        }));
        // Lambda Permissions - For serverless functions
        deployRole.addToPolicy(new iam.PolicyStatement({
            actions: [
                'lambda:UpdateFunctionCode',
                'lambda:UpdateFunctionConfiguration',
                'lambda:GetFunction',
                'lambda:InvokeFunction'
            ],
            resources: [
                `arn:aws:lambda:${this.region}:${this.account}:function:wishapp-*`
            ]
        }));
    }
}
exports.InfraStack = InfraStack;
//# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5mcmEtc3RhY2suanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlcyI6WyJpbmZyYS1zdGFjay50cyJdLCJuYW1lcyI6W10sIm1hcHBpbmdzIjoiOzs7QUFBQSxtQ0FBbUM7QUFDbkMsMkNBQTJDO0FBaUIzQzs7Ozs7O0dBTUc7QUFDSCxNQUFhLFVBQVcsU0FBUSxHQUFHLENBQUMsS0FBSztJQUN2QyxZQUFZLEtBQWdCLEVBQUUsRUFBVSxFQUFFLEtBQXdCO1FBQ2hFLEtBQUssQ0FBQyxLQUFLLEVBQUUsRUFBRSxFQUFFLEtBQUssQ0FBQyxDQUFDO1FBRXhCLDJEQUEyRDtRQUMzRCxvRUFBb0U7UUFDcEUseUNBQXlDO1FBQ3pDLE1BQU0sWUFBWSxHQUFHLEdBQUcsQ0FBQyxxQkFBcUIsQ0FBQyw0QkFBNEIsQ0FDekUsSUFBSSxFQUNKLG9CQUFvQixFQUNwQixnQkFBZ0IsSUFBSSxDQUFDLE9BQU8sb0RBQW9ELENBQ2pGLENBQUM7UUFFRiw2REFBNkQ7UUFDN0Qsd0VBQXdFO1FBQ3hFLCtDQUErQztRQUMvQyxvQ0FBb0M7UUFDcEMsTUFBTSxVQUFVLEdBQUcsSUFBSSxHQUFHLENBQUMsSUFBSSxDQUFDLElBQUksRUFBRSxrQkFBa0IsRUFBRTtZQUN4RCxTQUFTLEVBQUUsSUFBSSxHQUFHLENBQUMsa0JBQWtCLENBQ25DLFlBQVksQ0FBQyx3QkFBd0IsRUFDckM7Z0JBQ0UsVUFBVSxFQUFFO29CQUNWLENBQUMseUNBQXlDLENBQUMsRUFBRTt3QkFDM0MsUUFBUSxLQUFLLENBQUMsU0FBUyxJQUFJLEtBQUssQ0FBQyxVQUFVLGVBQWU7d0JBQzFELFFBQVEsS0FBSyxDQUFDLFNBQVMsSUFBSSxLQUFLLENBQUMsVUFBVSxzQkFBc0I7cUJBQ2xFO2lCQUNGO2FBQ0YsRUFDRCwrQkFBK0IsQ0FDaEM7WUFDRCxXQUFXLEVBQUUsMkNBQTJDO1lBQ3hELGtCQUFrQixFQUFFLEdBQUcsQ0FBQyxRQUFRLENBQUMsS0FBSyxDQUFDLENBQUMsQ0FBQztTQUMxQyxDQUFDLENBQUM7UUFFSCx1RUFBdUU7UUFDdkUsd0VBQXdFO1FBQ3hFLFVBQVUsQ0FBQyxXQUFXLENBQUMsSUFBSSxHQUFHLENBQUMsZUFBZSxDQUFDO1lBQzdDLE9BQU8sRUFBRTtnQkFDUCxjQUFjO2dCQUNkLGNBQWM7Z0JBQ2QsZUFBZTtnQkFDZixpQkFBaUI7YUFDbEI7WUFDRCxTQUFTLEVBQUU7Z0JBQ1QsK0JBQStCLElBQUksQ0FBQyxPQUFPLEVBQUU7Z0JBQzdDLCtCQUErQixJQUFJLENBQUMsT0FBTyxJQUFJO2FBQ2hEO1NBQ0YsQ0FBQyxDQUFDLENBQUM7UUFFSixzREFBc0Q7UUFDdEQsVUFBVSxDQUFDLFdBQVcsQ0FBQyxJQUFJLEdBQUcsQ0FBQyxlQUFlLENBQUM7WUFDN0MsT0FBTyxFQUFFO2dCQUNQLGtCQUFrQjtnQkFDbEIsa0JBQWtCO2dCQUNsQixxQkFBcUI7Z0JBQ3JCLHFCQUFxQjtnQkFDckIsZ0JBQWdCO2dCQUNoQixlQUFlO2FBQ2hCO1lBQ0QsU0FBUyxFQUFFO2dCQUNULG9CQUFvQixJQUFJLENBQUMsTUFBTSxJQUFJLElBQUksQ0FBQyxPQUFPLGtCQUFrQjthQUNsRTtTQUNGLENBQUMsQ0FBQyxDQUFDO1FBRUosOENBQThDO1FBQzlDLFVBQVUsQ0FBQyxXQUFXLENBQUMsSUFBSSxHQUFHLENBQUMsZUFBZSxDQUFDO1lBQzdDLE9BQU8sRUFBRTtnQkFDUCwrQkFBK0I7Z0JBQy9CLDRCQUE0QjtnQkFDNUIsK0JBQStCO2FBQ2hDO1lBQ0QsU0FBUyxFQUFFO2dCQUNULHVCQUF1QixJQUFJLENBQUMsT0FBTyxpQkFBaUI7YUFDckQ7U0FDRixDQUFDLENBQUMsQ0FBQztRQUVKLGdEQUFnRDtRQUNoRCxVQUFVLENBQUMsV0FBVyxDQUFDLElBQUksR0FBRyxDQUFDLGVBQWUsQ0FBQztZQUM3QyxPQUFPLEVBQUU7Z0JBQ1AsMkJBQTJCO2dCQUMzQixvQ0FBb0M7Z0JBQ3BDLG9CQUFvQjtnQkFDcEIsdUJBQXVCO2FBQ3hCO1lBQ0QsU0FBUyxFQUFFO2dCQUNULGtCQUFrQixJQUFJLENBQUMsTUFBTSxJQUFJLElBQUksQ0FBQyxPQUFPLHFCQUFxQjthQUNuRTtTQUNGLENBQUMsQ0FBQyxDQUFDO0lBQ04sQ0FBQztDQUNGO0FBekZELGdDQXlGQyIsInNvdXJjZXNDb250ZW50IjpbImltcG9ydCAqIGFzIGNkayBmcm9tICdhd3MtY2RrLWxpYic7XG5pbXBvcnQgKiBhcyBpYW0gZnJvbSAnYXdzLWNkay1saWIvYXdzLWlhbSc7XG5pbXBvcnQgeyBDb25zdHJ1Y3QgfSBmcm9tICdjb25zdHJ1Y3RzJztcblxuLyoqXG4gKiBQcm9wZXJ0aWVzIHJlcXVpcmVkIGZvciBjb25maWd1cmluZyB0aGUgV2lzaGFwcCBpbmZyYXN0cnVjdHVyZSBzdGFja1xuICovXG5pbnRlcmZhY2UgV2lzaGFwcFN0YWNrUHJvcHMgZXh0ZW5kcyBjZGsuU3RhY2tQcm9wcyB7XG4gIC8qKlxuICAgKiBHaXRIdWIgb3JnYW5pemF0aW9uIG5hbWUgd2hlcmUgdGhlIHJlcG9zaXRvcnkgaXMgaG9zdGVkXG4gICAqL1xuICByZWFkb25seSBnaXRodWJPcmc6IHN0cmluZztcbiAgLyoqXG4gICAqIE5hbWUgb2YgdGhlIEdpdEh1YiByZXBvc2l0b3J5XG4gICAqL1xuICByZWFkb25seSBnaXRodWJSZXBvOiBzdHJpbmc7XG59XG5cbi8qKlxuICogTWFpbiBpbmZyYXN0cnVjdHVyZSBzdGFjayBmb3IgdGhlIFdpc2hhcHAgYXBwbGljYXRpb25cbiAqIFxuICogVGhpcyBzdGFjayBjcmVhdGVzIHRoZSBuZWNlc3NhcnkgQVdTIHJlc291cmNlcyBmb3IgR2l0SHViIEFjdGlvbnMgZGVwbG95bWVudHM6XG4gKiAtIE9JREMgcHJvdmlkZXIgY29uZmlndXJhdGlvbiBmb3Igc2VjdXJlIEdpdEh1YiBBY3Rpb25zIGF1dGhlbnRpY2F0aW9uXG4gKiAtIElBTSByb2xlIHdpdGggYXBwcm9wcmlhdGUgcGVybWlzc2lvbnMgZm9yIGRlcGxveW1lbnRzXG4gKi9cbmV4cG9ydCBjbGFzcyBJbmZyYVN0YWNrIGV4dGVuZHMgY2RrLlN0YWNrIHtcbiAgY29uc3RydWN0b3Ioc2NvcGU6IENvbnN0cnVjdCwgaWQ6IHN0cmluZywgcHJvcHM6IFdpc2hhcHBTdGFja1Byb3BzKSB7XG4gICAgc3VwZXIoc2NvcGUsIGlkLCBwcm9wcyk7XG5cbiAgICAvLyBDcmVhdGUgT0lEQyAoT3BlbklEIENvbm5lY3QpIHByb3ZpZGVyIGZvciBHaXRIdWIgQWN0aW9uc1xuICAgIC8vIFRoaXMgZW5hYmxlcyBzZWN1cmUgYXV0aGVudGljYXRpb24gYmV0d2VlbiBHaXRIdWIgQWN0aW9ucyBhbmQgQVdTXG4gICAgLy8gd2l0aG91dCBzdG9yaW5nIGxvbmctbGl2ZWQgY3JlZGVudGlhbHNcbiAgICBjb25zdCBvaWRjUHJvdmlkZXIgPSBpYW0uT3BlbklkQ29ubmVjdFByb3ZpZGVyLmZyb21PcGVuSWRDb25uZWN0UHJvdmlkZXJBcm4oXG4gICAgICB0aGlzLFxuICAgICAgJ0dpdEh1Yk9JRENQcm92aWRlcicsXG4gICAgICBgYXJuOmF3czppYW06OiR7dGhpcy5hY2NvdW50fTpvaWRjLXByb3ZpZGVyL3Rva2VuLmFjdGlvbnMuZ2l0aHVidXNlcmNvbnRlbnQuY29tYFxuICAgICk7XG5cbiAgICAvLyBDcmVhdGUgZGVwbG95bWVudCByb2xlIHdpdGggR2l0SHViIE9JREMgdHJ1c3QgcmVsYXRpb25zaGlwXG4gICAgLy8gVGhpcyByb2xlIGNhbiBvbmx5IGJlIGFzc3VtZWQgYnkgR2l0SHViIEFjdGlvbnMgd29ya2Zsb3dzIHJ1bm5pbmcgb246XG4gICAgLy8gMS4gUHVsbCByZXF1ZXN0cyBpbiB0aGUgc3BlY2lmaWVkIHJlcG9zaXRvcnlcbiAgICAvLyAyLiBQdXNoIGV2ZW50cyB0byB0aGUgbWFpbiBicmFuY2hcbiAgICBjb25zdCBkZXBsb3lSb2xlID0gbmV3IGlhbS5Sb2xlKHRoaXMsICdHaXRIdWJEZXBsb3lSb2xlJywge1xuICAgICAgYXNzdW1lZEJ5OiBuZXcgaWFtLkZlZGVyYXRlZFByaW5jaXBhbChcbiAgICAgICAgb2lkY1Byb3ZpZGVyLm9wZW5JZENvbm5lY3RQcm92aWRlckFybixcbiAgICAgICAge1xuICAgICAgICAgIFN0cmluZ0xpa2U6IHtcbiAgICAgICAgICAgIFtgdG9rZW4uYWN0aW9ucy5naXRodWJ1c2VyY29udGVudC5jb206c3ViYF06IFtcbiAgICAgICAgICAgICAgYHJlcG86JHtwcm9wcy5naXRodWJPcmd9LyR7cHJvcHMuZ2l0aHViUmVwb306cHVsbF9yZXF1ZXN0YCxcbiAgICAgICAgICAgICAgYHJlcG86JHtwcm9wcy5naXRodWJPcmd9LyR7cHJvcHMuZ2l0aHViUmVwb306cmVmOnJlZnMvaGVhZHMvbWFpbmBcbiAgICAgICAgICAgIF1cbiAgICAgICAgICB9XG4gICAgICAgIH0sXG4gICAgICAgICdzdHM6QXNzdW1lUm9sZVdpdGhXZWJJZGVudGl0eSdcbiAgICAgICksXG4gICAgICBkZXNjcmlwdGlvbjogJ1JvbGUgZm9yIEdpdEh1YiBBY3Rpb25zIHRvIGRlcGxveSBXaXNoQXBwJyxcbiAgICAgIG1heFNlc3Npb25EdXJhdGlvbjogY2RrLkR1cmF0aW9uLmhvdXJzKDEpXG4gICAgfSk7XG5cbiAgICAvLyBDb25maWd1cmUgSUFNIHBlcm1pc3Npb25zIGZvbGxvd2luZyB0aGUgcHJpbmNpcGxlIG9mIGxlYXN0IHByaXZpbGVnZVxuICAgIC8vIFMzIFBlcm1pc3Npb25zIC0gRm9yIG1hbmFnaW5nIHN0YXRpYyBhc3NldHMgYW5kIHVzZXItdXBsb2FkZWQgY29udGVudFxuICAgIGRlcGxveVJvbGUuYWRkVG9Qb2xpY3kobmV3IGlhbS5Qb2xpY3lTdGF0ZW1lbnQoe1xuICAgICAgYWN0aW9uczogW1xuICAgICAgICAnczM6UHV0T2JqZWN0JyxcbiAgICAgICAgJ3MzOkdldE9iamVjdCcsXG4gICAgICAgICdzMzpMaXN0QnVja2V0JyxcbiAgICAgICAgJ3MzOkRlbGV0ZU9iamVjdCdcbiAgICAgIF0sXG4gICAgICByZXNvdXJjZXM6IFtcbiAgICAgICAgYGFybjphd3M6czM6Ojp3aXNoYXBwLWFzc2V0cy0ke3RoaXMuYWNjb3VudH1gLFxuICAgICAgICBgYXJuOmF3czpzMzo6Ondpc2hhcHAtYXNzZXRzLSR7dGhpcy5hY2NvdW50fS8qYFxuICAgICAgXVxuICAgIH0pKTtcblxuICAgIC8vIER5bmFtb0RCIFBlcm1pc3Npb25zIC0gRm9yIHVzZXIgZGF0YSBhbmQgd2lzaCBsaXN0c1xuICAgIGRlcGxveVJvbGUuYWRkVG9Qb2xpY3kobmV3IGlhbS5Qb2xpY3lTdGF0ZW1lbnQoe1xuICAgICAgYWN0aW9uczogW1xuICAgICAgICAnZHluYW1vZGI6UHV0SXRlbScsXG4gICAgICAgICdkeW5hbW9kYjpHZXRJdGVtJyxcbiAgICAgICAgJ2R5bmFtb2RiOlVwZGF0ZUl0ZW0nLFxuICAgICAgICAnZHluYW1vZGI6RGVsZXRlSXRlbScsXG4gICAgICAgICdkeW5hbW9kYjpRdWVyeScsXG4gICAgICAgICdkeW5hbW9kYjpTY2FuJ1xuICAgICAgXSxcbiAgICAgIHJlc291cmNlczogW1xuICAgICAgICBgYXJuOmF3czpkeW5hbW9kYjoke3RoaXMucmVnaW9ufToke3RoaXMuYWNjb3VudH06dGFibGUvd2lzaGFwcC0qYFxuICAgICAgXVxuICAgIH0pKTtcblxuICAgIC8vIENsb3VkRnJvbnQgUGVybWlzc2lvbnMgLSBGb3IgQ0ROIG1hbmFnZW1lbnRcbiAgICBkZXBsb3lSb2xlLmFkZFRvUG9saWN5KG5ldyBpYW0uUG9saWN5U3RhdGVtZW50KHtcbiAgICAgIGFjdGlvbnM6IFtcbiAgICAgICAgJ2Nsb3VkZnJvbnQ6Q3JlYXRlSW52YWxpZGF0aW9uJyxcbiAgICAgICAgJ2Nsb3VkZnJvbnQ6R2V0RGlzdHJpYnV0aW9uJyxcbiAgICAgICAgJ2Nsb3VkZnJvbnQ6VXBkYXRlRGlzdHJpYnV0aW9uJ1xuICAgICAgXSxcbiAgICAgIHJlc291cmNlczogW1xuICAgICAgICBgYXJuOmF3czpjbG91ZGZyb250Ojoke3RoaXMuYWNjb3VudH06ZGlzdHJpYnV0aW9uLypgXG4gICAgICBdXG4gICAgfSkpO1xuXG4gICAgLy8gTGFtYmRhIFBlcm1pc3Npb25zIC0gRm9yIHNlcnZlcmxlc3MgZnVuY3Rpb25zXG4gICAgZGVwbG95Um9sZS5hZGRUb1BvbGljeShuZXcgaWFtLlBvbGljeVN0YXRlbWVudCh7XG4gICAgICBhY3Rpb25zOiBbXG4gICAgICAgICdsYW1iZGE6VXBkYXRlRnVuY3Rpb25Db2RlJyxcbiAgICAgICAgJ2xhbWJkYTpVcGRhdGVGdW5jdGlvbkNvbmZpZ3VyYXRpb24nLFxuICAgICAgICAnbGFtYmRhOkdldEZ1bmN0aW9uJyxcbiAgICAgICAgJ2xhbWJkYTpJbnZva2VGdW5jdGlvbidcbiAgICAgIF0sXG4gICAgICByZXNvdXJjZXM6IFtcbiAgICAgICAgYGFybjphd3M6bGFtYmRhOiR7dGhpcy5yZWdpb259OiR7dGhpcy5hY2NvdW50fTpmdW5jdGlvbjp3aXNoYXBwLSpgXG4gICAgICBdXG4gICAgfSkpO1xuICB9XG59XG4iXX0=