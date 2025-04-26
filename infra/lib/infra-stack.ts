import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import * as apigw from 'aws-cdk-lib/aws-apigateway';

export interface InfraStackProps extends cdk.StackProps {
  assetPath?: string;
}

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: InfraStackProps) {
    super(scope, id, props);

    // Lambda function
    const wishLambda = new lambda.Function(this, 'WishHandler', {
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(props?.assetPath || '../target/lambda/wishlist_api'),
      handler: 'doesnt.matter',
    });

    // API Gateway
    new apigw.LambdaRestApi(this, 'WishApi', {
      handler: wishLambda,
    });
  }
}
