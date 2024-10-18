import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';
import {Duration} from "aws-cdk-lib";
import { Function } from 'aws-cdk-lib/aws-lambda';
export class LambdaEventConsumerStack extends cdk.Stack {

  readonly lambdaFunction: Function;

  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    this.lambdaFunction = new RustFunction(this, 'event-consumer', {
      functionName: 'lambda-line-bot-event-consumer-api',
      description: 'Lambda function for event consumer',
      memorySize: 128,
      timeout: Duration.seconds(5),
      manifestPath: './event_consumer/Cargo.toml',
      environment: {
        'RUST_LOG': 'DEBUG'
      }
    });
  }
}
