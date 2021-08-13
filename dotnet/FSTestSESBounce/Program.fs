// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open Amazon
open Amazon.SimpleEmail
open Amazon.SimpleEmail.Model

let subject = "Amazon SES test (AWS SDK for .NET)"
let textBody = "This email was sent through Amazon SES using the AWS SDK for .NET"

let sendEmail sender receiver = 
    async {
        let sendRequest = SendEmailRequest()
        sendRequest.Source <- sender
        sendRequest.Destination <- Destination(
            toAddresses = ResizeArray<string>([receiver])
        )
        sendRequest.Message <- Message()
        sendRequest.Message.Subject <- Content(subject)
        sendRequest.Message.Body <- Body(text = Content(data = textBody))
        use client = new AmazonSimpleEmailServiceClient(RegionEndpoint.USWest2)
        let! result = client.SendEmailAsync(request = sendRequest) |> Async.AwaitTask
        printfn "%s" (result.HttpStatusCode.ToString())
        printfn "Done"
    }
  
[<EntryPoint>]
let main argv =
    let senderAddress = argv.[0]
    let receiverAddress = argv.[1]
    sendEmail senderAddress receiverAddress 
        |> Async.RunSynchronously
    0