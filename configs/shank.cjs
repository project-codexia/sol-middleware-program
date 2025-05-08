const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
    generator: "shank",
    programName: "middleware_program",
    programId: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
    idlDir,
    idlName: "middleware_program",
    binaryInstallDir,
    programDir: path.join(programDir, "middleware-program"),
});
