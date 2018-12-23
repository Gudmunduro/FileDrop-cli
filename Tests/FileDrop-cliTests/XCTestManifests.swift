import XCTest

#if !os(macOS)
public func allTests() -> [XCTestCaseEntry] {
    return [
        testCase(FileDrop_cliTests.allTests),
    ]
}
#endif