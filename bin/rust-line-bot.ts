#!/usr/bin/env node
import 'source-map-support/register';
import * as dotenv from 'dotenv'
import * as cdk from 'aws-cdk-lib';
import {ApiGatewayLineBotStack} from '../lib/apigateway-line-bot-stack'
import {LambdaEventConsumerStack} from "../lib/lambda-event-consumer-stack";

dotenv.config();
const app = new cdk.App();

const lambdaEventConsumerStack = new LambdaEventConsumerStack(app, 'lambda-event-consumer-stack');
new ApiGatewayLineBotStack(app, 'apigateway-line-bot-stack', lambdaEventConsumerStack.lambdaFunction);

