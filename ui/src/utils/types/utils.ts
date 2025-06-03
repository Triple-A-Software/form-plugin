export type NullToUndefined<T> = T extends null
	? NonNullable<T> | undefined
	: T extends object
		? { [K in keyof T]: NullToUndefined<T[K]> }
		: T extends Array<infer U>
			? Array<NullToUndefined<U>>
			: T;

export type UndefinedToNull<T> = T extends undefined
	? Exclude<T, undefined> | null
	: T extends object
		? { [K in keyof T]: UndefinedToNull<T[K]> }
		: T extends Array<infer U>
			? Array<UndefinedToNull<U>>
			: T;
