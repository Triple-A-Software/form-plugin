export interface JsonObject extends Record<string, JsonValue> {}
export type JsonPrimitive = string | number | boolean | undefined;
export interface JsonArray extends Array<JsonValue> {}
export type JsonValue = JsonPrimitive | JsonArray | JsonObject;
