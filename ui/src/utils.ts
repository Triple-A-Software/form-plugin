import type { Property } from "../../bindings";

export function createElement<
	TType extends Property["type"],
	TOutput extends Extract<Property, { type: TType }>,
>(type: TType): TOutput {
	switch (type) {
		case "richtext":
		case "number":
		case "markdown":
		case "textarea":
		case "text":
		case "checkbox":
			return {
				type,
				class: "",
				label: {},
				required: false,
				name: "",
			} as TOutput;
		case "buttongroup":
			return {
				type,
				class: "",
				label: {},
				required: false,
				name: "",
				options: [],
			} as unknown as TOutput;
		case "select":
			return {
				type: "select",
				class: "",
				label: {},
				required: false,
				name: "",
				data_type: "string",
				options: [],
			} as unknown as TOutput;
		case "range":
			return {
				type,
				class: "",
				label: {},
				required: false,
				name: "",
				min: 0,
				max: 100,
			} as TOutput;
		case "code":
			return {
				type,
				class: "",
				label: {},
				required: false,
				name: "",
				language: "html",
			} as TOutput;
		default:
			// this is just the PropertyCommon
			return {
				type,
				class: "",
				label: {},
				required: false,
				name: "",
			} as TOutput;
	}
}
