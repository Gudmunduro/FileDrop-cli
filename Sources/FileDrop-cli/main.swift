import Foundation
import Just

let filename = CommandLine.arguments[1]
guard filename != "" else {
    print("Argument for filename not found")
    exit(1)
}

guard let idResponse = Just.get("https://filedrop.fisedush.com/api/drop/id").json as? [String:AnyObject], let dropID = idResponse["dropId"] as? String else {
    print("Failed to get dropID")
    exit(1)
}

let fileDirectory = (filename.hasPrefix("/")) ? URL(string: "file:///") : URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let fileLocation = URL(fileURLWithPath: filename, relativeTo: fileDirectory)

let uploadResponse = Just.post("https://filedrop.fisedush.com/api/drop/upload", data: ["dropId": dropID], files: ["file": .url(fileLocation, nil)])

print("Upload " + ((uploadResponse.ok) ? "succeeded" : "failed"))
print("Link: https://filedrop.fisedush.com/" + dropID)
print("Direct download: https://filedrop.fisedsuh.com/api/download/" + dropID)