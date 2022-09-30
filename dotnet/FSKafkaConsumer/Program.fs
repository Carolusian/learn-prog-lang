// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open Confluent.Kafka
open FsKafka

let log = Serilog.LoggerConfiguration().CreateLogger()
let handler (messages: ConsumeResult<string, string> []) = async {
    for m in messages do
        printfn "Received: %s" m.Message.Value 
}
let config topic = KafkaConsumerConfig.Create ("MyClientId", "kafka-host", [topic], "MyGroupId", AutoOffsetReset.Earliest)
let cfg1, cfg2 = config "topic1", config "topic2"


[<EntryPoint>]
let main argv = 
    async {
        use consumer1 = BatchedConsumer.Start(log, cfg1, handler)
        use consumer2 = BatchedConsumer.Start(log, cfg2, handler)
        use _ = KafkaMonitor(log).Start(consumer1.Inner, cfg1.Inner.GroupId)
        use _ = KafkaMonitor(log).Start(consumer2.Inner, cfg2.Inner.GroupId)
        return! Async.Parallel [consumer1.AwaitWithStopOnCancellation(); consumer2.AwaitWithStopOnCancellation()]
    } 
    |> Async.RunSynchronously 
    |> ignore
    0 // return an integer exit code