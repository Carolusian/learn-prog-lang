// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open System.IO
open System.Text.RegularExpressions

let replacePlaceHolders input =
    Regex.Replace(input, "{{([a-zA-Z0-9_]+)}}", "${$1}")
    

// Define a function to read the sql template
let readSQLTemplate filename = 
    seq { 
        use stream = File.OpenRead filename
        use reader = new StreamReader(stream)
        while not reader.EndOfStream do
            yield replacePlaceHolders(reader.ReadLine())
    }

[<EntryPoint>]
let main argv =
    let lines = readSQLTemplate @"/Users/cs/Downloads/hot-article-algo.sql"
    File.WriteAllLines (@"/Users/cs/Downloads/hot-article-migration.sql", lines) |> ignore
    0 // return an integer exit code