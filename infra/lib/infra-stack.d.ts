import * as cdk from "aws-cdk-lib";
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
export declare class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: WishappStackProps);
}
export {};
