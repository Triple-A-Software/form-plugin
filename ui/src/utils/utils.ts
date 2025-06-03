import type { NullToUndefined, UndefinedToNull } from "./types/utils";

export function nullToUndefined<T>(value: T): NullToUndefined<T> {
	if (value == null) {
		return undefined as NullToUndefined<T>;
	}
	if (Array.isArray(value)) {
		return value.map((v) => nullToUndefined(v)) as NullToUndefined<T>;
	}
	if (typeof value === "object") {
		return Object.fromEntries(
			Object.entries(value).map(([key, value]) => [
				key,
				nullToUndefined(value),
			]),
		) as NullToUndefined<T>;
	}
	return value as NullToUndefined<T>;
}

export function undefinedToNull<T>(value: T): UndefinedToNull<T> {
	if (value === undefined) {
		return undefined as UndefinedToNull<T>;
	}
	if (Array.isArray(value)) {
		return value.map((v) => undefinedToNull(v)) as UndefinedToNull<T>;
	}
	if (typeof value === "object" && value !== null) {
		return Object.fromEntries(
			Object.entries(value).map(([key, value]) => [
				key,
				undefinedToNull(value),
			]),
		) as UndefinedToNull<T>;
	}
	return value as UndefinedToNull<T>;
}
