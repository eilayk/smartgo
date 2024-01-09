deploying: 

az containerapp up --name smartgo-api --resource-group smartgo-rg --location canadacentral --environment 'smartgo-env' --ingress external --source . --env-vars "GO_API_KEY="

updating:

az containerapp update -n smartgo-api -g smartgo-rg --source .

