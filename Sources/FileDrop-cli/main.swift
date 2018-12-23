import Foundation
import Just

let filename = CommandLine.arguments[0]
guard filename != "" else {
    print("Argument for filename not found")
    exit(1)
}

guard let idResponse = Just.get("https://filedrop.fisedush.com/api/drop/id").json as? [String:AnyObject], let dropID = idResponse["dropId"] else {
    print("Failed to get dropID")
    exit(1)
}

let currentDirectoryURL = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let fileLocation = URL(fileURLWithPath: filename, relativeTo: currentDirectoryURL)

let uploadResponse = Just.post("https://filedrop.fisedush.com/api/drop/upload", json: ["dropId": dropID], files: ["file": .url(fileLocation, nil)])

print(uploadResponse.text ?? "Failed to get response")

print("Upload " + ((uploadResponse.ok) ? "succeeded" : "failed"))