import { cleanDoubleSlashes } from "ufo";
import type { Property, Submission } from "../../bindings";

export type PropertyType = Property["type"];
export type PropertySelector<TType extends PropertyType> = Extract<
    Property,
    { type: TType }
>;

export function createElement(type: PropertyType): Property | undefined {
    if (type === "button") return createButton();
    else if (type === "number") return createNumber();
    else if (type === "textarea") return createTextarea();
    else if (type === "text") return createText();
    else if (type === "checkbox") return createCheckbox();
    else if (type === "buttongroup") return createButtongroup();
    else if (type === "select") return createSelect();
    else if (type === "range") return createRange();
    else if (type === "captcha") return createCaptcha();
    else if (type === "date") return createDate();
    else if (type === "file") return createFile();
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
        default_value: undefined,
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
        label_class: undefined,
        input_class: undefined,
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
        label_class: undefined,
        input_class: undefined,
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
        label_class: undefined,
        input_class: undefined,
    };
}

export function createDate(): PropertySelector<"date"> {
    return {
        type: "date",
        class: "",
        label: {},
        hint: {},
        required: false,
        name: "",
        default_value: undefined,
        color: "neutral",
        placeholder: {},
        label_class: undefined,
        input_class: undefined,
        input_type: "date",
    };
}

export function createFile(): PropertySelector<"file"> {
    return {
        type: "file",
        class: "",
        label: {},
        hint: {},
        required: false,
        name: "",
        default_value: undefined,
        color: "neutral",
        label_class: undefined,
        input_class: undefined,
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
        label_class: undefined,
        input_class: undefined,
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
        button_class: undefined,
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
        label_class: undefined,
        input_class: undefined,
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
        label_class: undefined,
        input_class: undefined,
        min_label_class: undefined,
        max_label_class: undefined,
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

export function submissionFileDownloadLink(submission: Submission) {
    const a = document.createElement("a");
    a.href = cleanDoubleSlashes(
        `${__API_BASE__}/api/submissions/download-files?id=${submission.id}`,
    );
    a.click();
    a.remove();
}
