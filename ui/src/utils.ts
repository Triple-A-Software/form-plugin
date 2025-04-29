import type { Property } from "../../bindings";
import type { NullToUndefined } from "./utils/types/utils";

export type PropertyType = Property["type"];
export type PropertySelector<TType extends PropertyType> = NullToUndefined<
	Extract<Property, { type: TType }>
>;

export function createElement(
	type: PropertyType,
): NullToUndefined<Property> | undefined {
	if (type === "button") return createButton();
	else if (type === "number") return createNumber();
	else if (type === "textarea") return createTextarea();
	else if (type === "text") return createText();
	else if (type === "checkbox") return createCheckbox();
	else if (type === "buttongroup") return createButtongroup();
	else if (type === "select") return createSelect();
	else if (type === "range") return createRange();
	else if (type === "captcha") return createCaptcha();
}

export function createButton(): PropertySelector<"button"> {
	return {
		type: "button",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		button_type: "button",
		variant: "solid",
		default_value: null,
		color: "neutral",
	};
}

export function createNumber(): PropertySelector<"number"> {
	return {
		type: "number",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		min: undefined,
		max: undefined,
		step: undefined,
		placeholder: {},
		default_value: undefined,
		color: "neutral",
	};
}

export function createTextarea(): PropertySelector<"textarea"> {
	return {
		type: "textarea",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		max_length: undefined,
		placeholder: {},
		rows: undefined,
		columns: undefined,
		default_value: undefined,
		color: "neutral",
	};
}

export function createText(): PropertySelector<"text"> {
	return {
		type: "text",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		max: undefined,
		default_value: undefined,
		color: "neutral",
		placeholder: {},
		input_type: "text",
	};
}

export function createCheckbox(): PropertySelector<"checkbox"> {
	return {
		type: "checkbox",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		default_value: undefined,
		color: "neutral",
	};
}

export function createButtongroup(): PropertySelector<"buttongroup"> {
	return {
		type: "buttongroup",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		options: [],
		default_value: undefined,
		color: "neutral",
	};
}

export function createSelect(): PropertySelector<"select"> {
	return {
		type: "select",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		multiple: false,
		data_type: "string",
		options: [],
		default_value: undefined,
		color: "neutral",
	};
}

export function createRange(): PropertySelector<"range"> {
	return {
		type: "range",
		class: "",
		label: {},
		hint: {},
		required: false,
		name: "",
		min: 0,
		max: 100,
		default_value: undefined,
		color: "neutral",
		max_label: {},
		min_label: {},
		step: undefined,
	};
}

// this function is not exported, because the captcha element doesn't have a
// specific form, it is configured through the form of the form
function createCaptcha(): PropertySelector<"captcha"> {
	return {
		type: "captcha",
		captchaClass: "",
		sitekey: "",
		// this is just a placeholder type
		captchaType: "h-captcha",
		callback: "",
		expiredCallback: "",
	};
}
