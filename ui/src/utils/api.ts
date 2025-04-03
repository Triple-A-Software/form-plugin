import {
	type UseMutationOptions,
	type UseQueryOptions,
	useMutation,
	useQuery,
} from "@pinia/colada";
import type { API } from "./types/api";
import { err, ok, ResultAsync, type Result } from "neverthrow";
import type { ApiError } from "../../../bindings";

type Body<T> = T extends { body: infer TBody } ? TBody : never;
type Query<T> = T extends { query: infer TQuery } ? TQuery : never;
type Options<T, TMethod> = T extends { body: infer TBody; query: infer TQuery }
	? { body: TBody; query: TQuery; method: TMethod }
	: T extends { body: infer TBody }
		? { body: TBody; method: TMethod }
		: T extends { query: infer TQuery }
			? { query: TQuery; method: TMethod }
			: { method: TMethod };
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
	TBody extends Body<API[TPath][TMethod]>,
	TQuery extends Query<API[TPath][TMethod]>,
>(
	path: TPath,
	options: Options<API[TPath][TMethod], TMethod>,
	fetchOptions?: Omit<RequestInit, "body" | "method">,
): Promise<Result<TResponse, ApiError>> {
	const _options = options as {
		body?: TBody;
		query?: TQuery;
		method?: TMethod;
	};
	const query = new URLSearchParams();
	for (const [key, value] of Object.entries(_options.query ?? {})) {
		if (Array.isArray(value)) {
			for (const v of value) {
				query.append(key, v);
			}
			if (value.length === 0) {
				query.append(key, "");
			}
		} else if (value) {
			query.set(key, String(value));
		}
	}
	let _path = path as string;
	if (_options.query) {
		_path += `?${query.toString()}`;
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
	return ok(json as TResponse);
}

// biome-ignore lint/suspicious/noExplicitAny: because we don't know the type
function isApiError(json: any): json is ApiError {
	return !!json.error_message;
}

export function useApiQuery<
	TPath extends keyof API,
	TMethod extends keyof API[TPath],
	TResponse extends ExtractResponse<API[TPath][TMethod]>,
	TBody extends Body<API[TPath][TMethod]>,
	TQuery extends Query<API[TPath][TMethod]>,
	TQueryOptions extends Omit<
		UseQueryOptions<TResponse, ApiError, TResponse>,
		"query"
	>,
>(
	path: (() => TPath) | TPath,
	options: Options<API[TPath][TMethod], TMethod>,
	queryOptions?: TQueryOptions,
) {
	return useQuery({
		key: () => [typeof path === "string" ? path : path()],
		query: async () => {
			const result = await fetchApi<TPath, TMethod, TResponse, TBody, TQuery>(
				typeof path === "string" ? path : path(),
				options,
			);
			return result.match(
				(ok) => {
					return ok;
				},
				(err) => {
					throw err;
				},
			);
		},
		...queryOptions,
	});
}

export function useApiMutation<
	TPath extends keyof API,
	TMethod extends keyof API[TPath],
	TResponse extends ExtractResponse<API[TPath][TMethod]>,
	TBody extends Body<API[TPath][TMethod]>,
	TQuery extends Query<API[TPath][TMethod]>,
	TMutationOptions extends Omit<
		UseMutationOptions<Result<TResponse, ApiError>, TBody, Error, {}>,
		"mutation"
	>,
>(
	path: TPath,
	options: Omit<Options<API[TPath][TMethod], TMethod>, "body">,
	mutationOptions?: TMutationOptions,
) {
	return useMutation({
		mutation: (data: TBody) =>
			fetchApi<TPath, TMethod, TResponse, TBody, TQuery>(path, {
				...options,
				body: data,
			} as Options<API[TPath][TMethod], TMethod>),
		...mutationOptions,
	});
}
