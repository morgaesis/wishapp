import * as cdk from "aws-cdk-lib";
// Temporary comment to trigger CI/CD
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as apigw from "aws-cdk-lib/aws-apigateway";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";

export interface InfraStackProps extends cdk.StackProps {
  assetPath?: string;
}

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: InfraStackProps) {
    super(scope, id, props);

    // DynamoDB Table
    const wishlistTable = new dynamodb.Table(this, "WishlistTable", {
      tableName: "wishlist_table",
      partitionKey: { name: "id", type: dynamodb.AttributeType.STRING },
      removalPolicy: cdk.RemovalPolicy.DESTROY, // NOT recommended for production code
    });

    // Lambda function
    const wishLambda = new lambda.Function(this, "WishHandler", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(
        props?.assetPath || "../target/lambda/wishlist_api",
      ),
      handler: "doesnt.matter",
      environment: {
        DUMMY_VAR: "1",
        TABLE_NAME: wishlistTable.tableName,
      },
    });

    // Grant Lambda permissions to read/write from the DynamoDB table
    wishlistTable.grantReadWriteData(wishLambda);

    // API Gateway
    new apigw.LambdaRestApi(this, "WishApi", {
      handler: wishLambda,
    });
  }
}
