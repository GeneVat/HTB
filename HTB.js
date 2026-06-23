const fs = require("fs");
const path = require("path");
const readline = require("readline");
const { exec } = require("child_process");

const INPUT_FILE = path.join(__dirname, "./main.html");
const OUTPUT_FILE = path.join(__dirname, "./output.bbcode");

const logError = (msg) => console.error(`\x1b[31;1m${msg}\x1b[0m`);
const logSuccess = (msg) => console.log(`\x1b[32m${msg}\x1b[0m`);
const logInfo = (msg) => console.log(`\x1b[34m${msg}\x1b[0m`);

// Minimal HTML to BBCode mapping for demonstration
const BBCODE_MAP = [
	// [htmlTag, bbcodeTag]
	// Floating
codes.insert("float", "float");
	// Alignment
codes.insert("align", "align");
	// Indentation
codes.insert("tab", "tab");
	// Box
codes.insert("box", "box");
	// Sidebar
codes.insert("sidebar", "sidebar");
codes.insert("blockquote", "sidebar");
	// Bold
codes.insert("b", "b");
codes.insert("strong", "b");
	// Italic
codes.insert("i", "i");
codes.insert("em", "i");
	// Underline
codes.insert("u", "u");
	// Superscript
codes.insert("sup", "sup");
	// Subscript
codes.insert("sub", "sub");
	// Strike
codes.insert("strike", "strike");
codes.insert("del", "strike");
codes.insert("s", "strike");
	// Unformatted Text
codes.insert("pre", "pre");
	// Code (Forum Only)
codes.insert("code", "code");
	// Horizontal Ruler
codes.insert("hr", "hr");
	// Links
codes.insert("a", "url");
	// Anchors
codes.insert("anchor", "anchor");
	// Nations
codes.insert("nation", "nation");
	// Regions
codes.insert("region", "region");
	// WA Proposal
codes.insert("proposal", "proposal");
	// WA Resolution
codes.insert("resolution", "resolution");
	// Quote
codes.insert("quote", "quote");
	// Size
codes.insert("title", "size=180");

codes.insert("h1", "size=150");


];

// Simple HTML to BBCode converter (expand as needed)
function htmlToBBCode(html) {
	let output = html;

	// Replace tags with BBCode equivalents
	BBCODE_MAP.forEach(([htmlTag, bbTag]) => {
		// Use \b to ensure only the exact tag is matched (not tags like <span> for <s>)
		const openTag = new RegExp(`<${htmlTag}\\b(\\s[^>]*)?>`, "gi");
		const closeTag = new RegExp(`</${htmlTag}\\b>`, "gi");
		// Add support for id="*" -> [tag=*]
		const openTagWithId = new RegExp(`<${htmlTag}\\b([^>]*)id=("']([^"']+)("']([^>]*)>`, "gi");

		if (bbTag === "*") {
			// List item: [*]content
			output = output.replace(openTag, "[*]");
			output = output.replace(closeTag, "");
		} else if (bbTag.startsWith("size=")) {
			const size = bbTag.split("=")[1];
			output = output.replace(openTag, `[size=${size}]`);
			output = output.replace(closeTag, "[/size]");
		} else if (bbTag === "img") {
			// [img]src[/img]
			output = output.replace(/<img\b[^>]*src=("']([^"']+)("'][^>]*>/gi, "[img]$1[/img]");
		} else if (bbTag === "url") {
			// [url=href]text[/url]
			output = output.replace(/<a\b[^>]*href=("']([^"']+)("'][^>]*>(.*?)<\/a>/gi, "[url=$1]$2[/url]");
		} else if (bbTag === "hr") {
			output = output.replace(openTag, "[hr]");
			output = output.replace(closeTag, "");
		} else {
			// Handle id attribute for all tags
			output = output.replace(openTagWithId, function(_, before, idValue, after) {
				return `[${bbTag}=${idValue}]`;
			});
			output = output.replace(openTag, `[${bbTag}]`);
			output = output.replace(closeTag, `[/${bbTag}]`);
		}
	});

	// Remove any remaining HTML tags
	output = output.replace(/<[^>]+>/g, "");

	// Unescape HTML entities (basic)
	output = output.replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&amp;/g, "&");

	return output.trim();
}

function convert() {
	if (!fs.existsSync(INPUT_FILE)) {
		logError(`Input file not found: ${INPUT_FILE}`);
		return false;
	}
	try {
		const html = fs.readFileSync(INPUT_FILE, "utf8");
		const bbcode = htmlToBBCode(html);
		fs.writeFileSync(OUTPUT_FILE, bbcode);
		logSuccess(`Conversion successful! Output: ${OUTPUT_FILE}`);
		return true;
	} catch (err) {
		logError(`Conversion failed: ${err.message}`);
		return false;
	}
}

function showHelp() {
	logInfo(`
HTB (HTML to BBCode) Commands:
  r  - Re-run conversion (${path.basename(INPUT_FILE)} -> ${path.basename(OUTPUT_FILE)})
  c  - Clear console
  q  - Quit the program
  h  - Show this help menu
  l  - List output file and attempt to open output directory (${path.dirname(OUTPUT_FILE)})
`);
}

function listOutputFiles() {
	if (!fs.existsSync(OUTPUT_FILE)) {
		logError(`Output file does not exist: ${OUTPUT_FILE}`);
		return;
	}
	logInfo("Output file:");
	console.log(`- ${OUTPUT_FILE}`);
	const openCommands = {
		darwin: `open "${path.dirname(OUTPUT_FILE)}"`,
		win32: `start "" "${path.dirname(OUTPUT_FILE)}"`,
		linux: `xdg-open "${path.dirname(OUTPUT_FILE)}"`,
	};
	const openCommand = openCommands[process.platform];
	if (!openCommand) {
		logError("Unsupported platform for opening directory automatically.");
		return;
	}
	logInfo(`Attempting to open directory: ${path.dirname(OUTPUT_FILE)}...`);
	exec(openCommand, (error, stdout, stderr) => {
		if (error) {
			logError(`Failed to open directory: ${error.message}`);
			logError(`stderr: ${stderr}`);
			return;
		}
		if (stderr) logInfo(`Open command stderr: ${stderr}`);
		logInfo(`Directory should be open.`);
	});
}

// --- Main Execution ---
console.clear();
logSuccess("HTB (HTML to BBCode) Running.");
logInfo(`Watching ${INPUT_FILE} for changes.`);
logInfo("Press 'h' for help.");

if (convert()) {
	// Initial conversion success
} else {
	logError("Initial conversion failed. Please check main.html");
}

// Debounce file watcher to prevent rapid re-converts
let convertTimeout = null;
fs.watch(INPUT_FILE, { encoding: "utf8" }, (eventType) => {
	if (eventType === "change") {
		clearTimeout(convertTimeout);
		convertTimeout = setTimeout(() => {
			logInfo(`\nFile change detected in ${path.basename(INPUT_FILE)}. Re-converting...`);
			if (convert()) {
				logSuccess("Re-conversion complete.");
			} else {
				logError("Re-conversion failed.");
			}
			rl.prompt();
		}, 250);
	}
});

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stdout,
	prompt: '> '
});

rl.prompt();

rl.on("line", (input) => {
	const command = input.trim().toLowerCase();
	switch (command) {
		case "q":
		case "exit":
			logInfo("Exiting HTB (HTML to BBCode).");
			rl.close();
			process.exit(0);
			break;
		case "h":
		case "help":
			showHelp();
			break;
		case "r":
		case "reconvert":
			logInfo("Manual conversion requested...");
			if (convert()) logSuccess("File converted successfully.");
			else logError("Manual conversion failed.");
			break;
		case "c":
		case "clear":
			console.clear();
			logSuccess("HTB (HTML to BBCode) Running.");
			logInfo(`Watching ${INPUT_FILE} for changes.`);
			logInfo("Press 'h' for help.");
			break;
		case "l":
		case "list":
			listOutputFiles();
			break;
		default:
			logInfo(`Unknown command: "${input.trim()}". Press 'h' for help.`);
			break;
	}
	rl.prompt();
}).on('close', () => {
	logInfo('HTB (HTML to BBCode) stopped.');
	process.exit(0);
});
