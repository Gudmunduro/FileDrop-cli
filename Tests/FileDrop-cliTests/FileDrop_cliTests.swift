import XCTest
@testable import FileDrop_cli

final class FileDrop_cliTests: XCTestCase {
    func testExample() {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertEqual(FileDrop_cli().text, "Hello, World!")
    }

    static var allTests = [
        ("testExample", testExample),
    ]
}
