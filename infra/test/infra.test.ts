import * as cdk from 'aws-cdk-lib';
import { Template } from 'aws-cdk-lib/assertions';
import * as Infra from '../lib/infra-stack';

describe('Infrastructure Stack', () => {
  let app: cdk.App;
  let stack: Infra.InfraStack;
  let template: Template;

  beforeAll(() => {
    app = new cdk.App();
    stack = new Infra.InfraStack(app, 'TestStack', {
      githubOrg: 'test-org',
      githubRepo: 'test-repo'
    });
    template = Template.fromStack(stack);
  });

  test('Creates SQS Queue with correct configuration', () => {
    template.hasResourceProperties('AWS::SQS::Queue', {
      VisibilityTimeout: 300
    });
  });

  test('Creates IAM Role for GitHub OIDC', () => {
    template.hasResourceProperties('AWS::IAM::Role', {
      AssumeRolePolicyDocument: {
        Statement: [{
          Action: 'sts:AssumeRoleWithWebIdentity',
          Effect: 'Allow',
          Condition: {
            StringLike: {
              'token.actions.githubusercontent.com:sub': [
                `repo:test-org/test-repo:pull_request`,
                `repo:test-org/test-repo:ref:refs/heads/main`
              ]
            }
          }
        }]
      },
      MaxSessionDuration: 3600
    });
  });

  test('Has correct CDK Output exports', () => {
    template.hasOutput('GitHubOidcRoleArn', {
      Export: {
        Name: 'GitHubOidcRoleArn'
      }
    });
  });

  test('Verifies CloudFront permissions exist', () => {
    const policies = template.findResources('AWS::IAM::Policy');
    const cloudfrontActions = [
      'cloudfront:CreateInvalidation',
      'cloudfront:GetDistribution',
      'cloudfront:UpdateDistribution'
    ];
    
    expect(
      Object.values<any>(policies).some((policy: any) => 
        policy.Properties?.PolicyDocument?.Statement?.some((statement: any) =>
          Array.isArray(statement.Action)
            ? statement.Action.some((action: any) => 
                typeof action === 'string' 
                  ? cloudfrontActions.includes(action)
                  : action.some((a: string) => cloudfrontActions.includes(a))
              )
            : false
        )
      )
    ).toBeTruthy();
  });
});
