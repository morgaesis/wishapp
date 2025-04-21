import * as cdk from "aws-cdk-lib";
import { Match, Template } from "aws-cdk-lib/assertions";
import * as Infra from "../lib/infra-stack";

describe("Infrastructure Stack", () => {
  describe("Production Stack", () => {
    let template: Template;

    beforeAll(() => {
      const app = new cdk.App({
        context: { githubRepo: "morgaesis/wishapp" }
      });
      const stack = new Infra.InfraStack(app, "ProdStack", {
        env: { account: "123456789012", region: "us-east-1" },
        githubOrg: "morgaesis",
        githubRepo: "wishapp"
      });
      template = Template.fromStack(stack);
    });

    test("Creates GitHub Actions role with correct trust policy", () => {
      template.hasResourceProperties("AWS::IAM::Role", {
        RoleName: "github-actions-prod",
        AssumeRolePolicyDocument: {
          Statement: [
            Match.objectLike({
              Condition: {
                StringLike: {
                  "token.actions.githubusercontent.com:sub": [
                    "repo:morgaesis/wishapp:pull_request",
                    "repo:morgaesis/wishapp:ref:refs/heads/main"
                  ]
                }
              }
            })
          ]
        },
        MaxSessionDuration: 3600
      });
    });

    test("Includes tag-based access controls", () => {
      template.hasResourceProperties("AWS::IAM::Role", {
        Policies: Match.arrayWith([
          Match.objectLike({
            PolicyDocument: {
              Statement: Match.arrayWith([
                Match.objectLike({
                  Condition: {
                    ArnEquals: {
                      "aws:ResourceTag/StackType": "wishapp-prod"
                    }
                  }
                })
              ])
            }
          })
        ])
      });
    });
  });

  describe("PR Stack", () => {
    let template: Template;

    beforeAll(() => {
      const app = new cdk.App({
        context: {
          githubRepo: "morgaesis/wishapp",
          prNumber: "123"
        }
      });
      const stack = new Infra.InfraStack(app, "PRStack", {
        env: { account: "123456789012", region: "us-east-1" },
        githubOrg: "morgaesis",
        githubRepo: "wishapp"
      });
      template = Template.fromStack(stack);
    });

    test("Creates PR-specific role", () => {
      template.hasResourceProperties("AWS::IAM::Role", {
        RoleName: "github-actions-pr-123"
      });
    });

    test("Includes PR-specific tags", () => {
      template.hasResourceProperties("AWS::IAM::Role", {
        Policies: Match.arrayWith([
          Match.objectLike({
            PolicyDocument: {
              Statement: Match.arrayWith([
                Match.objectLike({
                  Condition: {
                    ArnEquals: {
                      "aws:ResourceTag/StackType": "wishapp-pr-123"
                    }
                  }
                })
              ])
            }
          })
        ])
      });
    });
  });
});
