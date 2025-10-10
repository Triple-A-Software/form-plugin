echo "Fixing generated types...";
# replace null with undefined in the generated types
find ./bindings -type f -name "*.ts" -exec sed -i 's/null/undefined/g' {} \;

# the recursive JsonObject type only works if it is a Record, not with { [key: string]: JsonValue }
echo "export interface JsonObject extends Record<string, JsonValue> {}" > ./bindings/serde_json/JsonValue.ts;
echo "export type JsonPrimitive = string | number | boolean | undefined;" >> ./bindings/serde_json/JsonValue.ts;
echo "export interface JsonArray extends Array<JsonValue> {}" >> ./bindings/serde_json/JsonValue.ts;
echo "export type JsonValue = JsonPrimitive | JsonArray | JsonObject;" >> ./bindings/serde_json/JsonValue.ts;

echo "Done!";

