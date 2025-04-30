import {
	type UseMutationOptions,
	type UseQueryOptions,
	useMutation,
	useQuery,
	useQueryCache,
} from "@pinia/colada";
import type { API } from "./types/api";
import { err, ok, ResultAsync, type Result } from "neverthrow";
import type { ApiError } from "../../../bindings";
import { nullToUndefined, undefinedToNull } from "./utils";
import type { NullToUndefined } from "./types/utils";
import { toValue, type MaybeRefOrGetter } from "vue";
import { useI18n } from "vue-i18n";

type Body<T> = T extends { body: infer TBody } ? TBody : never;
type Query<T> = T extends { query: infer TQuery } ? TQuery : never;
type Options<T, TMethod> = T extends { body: infer TBody; query: infer TQuery }
	? {
			body: MaybeRefOrGetter<TBody>;
			query: MaybeRefOrGetter<TQuery>;
			method: MaybeRefOrGetter<TMethod>;
		}
	: T extends { body: infer TBody }
		? { body: MaybeRefOrGetter<TBody>; method: MaybeRefOrGetter<TMethod> }
		: T extends { query: infer TQuery }
			? { query: MaybeRefOrGetter<TQuery>; method: MaybeRefOrGetter<TMethod> }
			: { method: MaybeRefOrGetter<TMethod> };
type MutationOptions<T, TMethod> = T extends { query: infer TQuery }
	? {
			query: MaybeRefOrGetter<TQuery>;
			method: MaybeRefOrGetter<TMethod>;
		}
	: { method: MaybeRefOrGetter<TMethod> };
type ExtractResponse<T> = Extract<T, { response: unknown }>["response"];

export const safeFetch = ResultAsync.fromThrowable(fetch, (error) => {
	if (typeof error === "string" && error === "resend") {
		return {
			error_message: "request has been aborted by client",
		} satisfies ApiError;
	}
	if (
		error &&
		typeof error === "object" &&
		"name" in error &&
		error.name === "AbortError"
	) {
		return {
			error_message: "request has been aborted by client",
		} satisfies ApiError;
	}
	return {
		error_message: "unknown_error",
	} satisfies ApiError;
});

export async function fetchApi<
	TPath extends keyof API,
	TMethod extends keyof API[TPath],
	TResponse extends ExtractResponse<API[TPath][TMethod]>,
	TBody extends NullToUndefined<Body<API[TPath][TMethod]>>,
	TQuery extends NullToUndefined<Query<API[TPath][TMethod]>>,
>(
	path: TPath,
	options: Options<API[TPath][TMethod], TMethod>,
	fetchOptions?: Omit<RequestInit, "body" | "method">,
): Promise<Result<NullToUndefined<TResponse>, ApiError>> {
	const _options = undefinedToNull({
		body: "body" in options ? toValue(options.body) : undefined,
		query: "query" in options ? toValue(options.query) : undefined,
		method: toValue(options.method),
	} as {
		body?: TBody;
		query?: TQuery;
		method?: TMethod;
	});
	const query = new URLSearchParams();
	for (const [key, value] of Object.entries(_options.query ?? {})) {
		if (Array.isArray(value)) {
			for (const v of value) {
				query.append(key, v);
			}
			if (value.length === 0) {
				query.append(key, "");
			}
		} else if (value != null) {
			query.set(key, String(value));
		}
	}
	let _path = path as string;
	if (_options.query) {
		_path += `?${query.toString()}`;
	}
	if (import.meta.env.MODE === "production") {
		_path = `${__API_BASE__}${_path}`;
	}
	let response: Awaited<ReturnType<typeof safeFetch>> | null = null;
	if (_options.body instanceof FormData) {
		response = await safeFetch(_path, {
			method: options.method as string,
			body: _options.body,
		});
	} else {
		response = await safeFetch(_path, {
			...fetchOptions,
			method: options.method as string,
			headers: {
				...fetchOptions?.headers,
				"Content-Type": "application/json",
			},
			body: JSON.stringify(_options.body),
		});
	}
	if (response.isErr()) {
		return err(response.error);
	}
	const json = await response.value.json();
	if (response.value.status < 200 || response.value.status >= 400) {
		return err(json);
	}
	if (isApiError(json)) {
		return err(json);
	}
	return ok(nullToUndefined(json as TResponse));
}

// biome-ignore lint/suspicious/noExplicitAny: because we don't know the type
function isApiError(json: any): json is ApiError {
	return !!json.error_message;
}

export function useApiQuery<
	TPath extends keyof API,
	TMethod extends keyof API[TPath],
	TResponse extends ExtractResponse<API[TPath][TMethod]>,
	TBody extends NullToUndefined<Body<API[TPath][TMethod]>>,
	TQuery extends NullToUndefined<Query<API[TPath][TMethod]>>,
	TQueryOptions extends Omit<
		UseQueryOptions<
			NullToUndefined<TResponse>,
			ApiError,
			NullToUndefined<TResponse>
		>,
		"query" | "key"
	>,
>(
	path: (() => TPath) | TPath,
	options: Options<API[TPath][TMethod], TMethod>,
	queryOptions?: TQueryOptions,
) {
	const _path = () => (typeof path === "string" ? path : path());
	const key = () => [`${_path()}:${String(options.method)}`];
	const queryCache = useQueryCache();
	const invalidate = () => {
		queryCache.invalidateQueries({ key: key() });
	};
	const toast = useToast();
	const { t, te } = useI18n();
	return {
		...useQuery({
			key,
			query: async () => {
				const result = await fetchApi<TPath, TMethod, TResponse, TBody, TQuery>(
					_path(),
					options,
				);
				return result.match(
					(ok) => {
						const tKey = `toast.success.${_path()}.${String(options.method)}`;
						if (te(tKey)) {
							toast.add({
								title: t(tKey),
								color: "success",
								icon: "i-tabler-check",
							});
						}
						return ok;
					},
					(err) => {
						const tKey = `toast.error.${_path()}.${String(options.method)}`;
						if (te(tKey)) {
							toast.add({
								title: t(tKey),
								color: "error",
								icon: "i-tabler-alert-circle",
							});
						}
						throw err;
					},
				);
			},
			...queryOptions,
		}),
		key,
		invalidate,
	};
}

export function useApiMutation<
	TPath extends keyof API,
	TMethod extends keyof API[TPath],
	TResponse extends ExtractResponse<API[TPath][TMethod]>,
	TBody extends NullToUndefined<Body<API[TPath][TMethod]>>,
	TQuery extends NullToUndefined<Query<API[TPath][TMethod]>>,
	TMutationOptions extends Omit<
		UseMutationOptions<
			Result<NullToUndefined<TResponse>, ApiError>,
			TBody,
			Error,
			{}
		>,
		"mutation"
	> & { invalidate?: Array<{ invalidate: () => void }> },
>(
	path: TPath,
	options: MutationOptions<API[TPath][TMethod], TMethod>,
	mutationOptions?: TMutationOptions,
) {
	const onSettled = mutationOptions?.invalidate
		? () => {
				if (!mutationOptions.invalidate) return;
				for (const query of mutationOptions.invalidate) {
					query.invalidate();
				}
			}
		: undefined;
	const toast = useToast();
	const { t, te } = useI18n();
	return useMutation({
		mutation: async (data: TBody) => {
			const response = await fetchApi<TPath, TMethod, TResponse, TBody, TQuery>(
				path,
				{
					...options,
					body: data,
				} as Options<API[TPath][TMethod], TMethod>,
			);
			response.match(
				(_) => {
					const tKey = `toast.success.${path}.${String(options.method)}`;
					if (te(tKey)) {
						toast.add({
							title: t(tKey),
							color: "success",
							icon: "i-tabler-check",
						});
					}
				},
				(_) => {
					const tKey = `toast.error.${path}.${String(options.method)}`;
					if (te(tKey)) {
						toast.add({
							title: t(tKey),
							color: "error",
							icon: "i-tabler-alert-circle",
						});
					}
				},
			);
			return response;
		},
		onSettled,
		...mutationOptions,
	});
}
