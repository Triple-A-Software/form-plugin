export function convertToCSV(arr) {
	// Collect all unique keys from all objects
	const allKeys = new Set();
	function extractKeys(obj, prefix) {
		Object.entries(obj).forEach(([key, value]) => {
			let _key = key;
			if (prefix) {
				_key = `${prefix}.${key}`;
			}
			if (typeof value === "object" && value != null && !Array.isArray(value)) {
				extractKeys(value, _key);
			} else {
				allKeys.add(_key);
			}
		});
	}
	arr.forEach((obj) => {
		extractKeys(obj);
	});
	const headers = Array.from(allKeys);

	// Create the header row
	const csvRows = [headers.join(",")];

	// Create the data rows
	for (const row of arr) {
		const values = headers.map((header) => {
			const headerPath = header.split(".");
			const value = headerPath.reduce((obj, key) => obj[key], row);
			// Escape double quotes by doubling them
			const escapedValue =
				typeof value === "string" ? value.replace(/"/g, '""') : value;
			// Wrap values containing commas or double quotes in double quotes
			return typeof value === "string" &&
				(value.includes(",") || value.includes('"'))
				? `"${escapedValue}"`
				: escapedValue;
		});
		csvRows.push(values.join(","));
	}

	// Join all rows with newline characters
	return csvRows.join("\n");
}
