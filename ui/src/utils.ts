import { cleanDoubleSlashes } from "ufo";
import type { Form, Property, Submission } from "../../bindings";

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
        multiple: undefined,
        accept: undefined,
        max_file_size_mb: undefined,
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

// elements whose value ends up in the submission data, so emails and integrations can reference them
export type DataProperty = Exclude<Property, { type: "captcha" | "button" }>;
export function dataFields(props: Property[]): DataProperty[] {
    return props.filter((p): p is DataProperty => p.type !== "captcha" && p.type !== "button");
}

export function slugify(text: string) {
    return text
        .normalize("NFKD")
        .replace(/[\u0300-\u036f]/g, "")
        .replace(/ß/g, "ss")
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, "_")
        .replace(/^_+|_+$/g, "");
}

export function findEmailField(props: Property[]) {
    return props.find((p): p is PropertySelector<"text"> => p.type === "text" && p.input_type === "email");
}

// path is "name", "props.<index>" or another key of Form, key is a translation key below "validation"
export type FormIssue = { path: string; key: string; params?: Record<string, string | number> };

function isBlank(value: string | number | undefined | null) {
    return value == null || String(value).trim() === "";
}

export function validateForm(form: Form): FormIssue[] {
    const issues: FormIssue[] = [];
    if (isBlank(form.name)) issues.push({ path: "name", key: "validation.required" });

    const seen = new Set<string>();
    form.props.forEach((prop, index) => {
        if (prop.type === "captcha" || prop.type === "button") return;
        if (isBlank(prop.name)) {
            issues.push({ path: `props.${index}`, key: "validation.fieldName" });
        } else if (seen.has(prop.name)) {
            issues.push({ path: `props.${index}`, key: "validation.duplicateName", params: { name: prop.name } });
        }
        seen.add(prop.name);
    });
    const captchas = form.props.filter((p) => p.type === "captcha").length;
    if (captchas > 1) issues.push({ path: "props", key: "validation.multipleCaptchas" });
    // the server verifies when activate_captcha is set, but only the element renders the widget
    if (form.activate_captcha && captchas === 0) issues.push({ path: "activate_captcha", key: "validation.captchaMissing" });
    if (!form.activate_captcha && captchas > 0) issues.push({ path: "activate_captcha", key: "validation.captchaUnchecked" });

    const required = (path: keyof Form, value: string | number | undefined) => {
        if (isBlank(value)) issues.push({ path, key: "validation.required" });
    };
    const fieldRef = (path: keyof Form, value: string | undefined, isRequired: boolean) => {
        if (value == null || isBlank(value)) {
            if (isRequired) issues.push({ path, key: "validation.required" });
        } else if (!seen.has(value)) {
            issues.push({ path, key: "validation.unknownField", params: { name: value } });
        }
    };

    if (form.send_confirmation_email) {
        required("confirmation_email_id", form.confirmation_email_id);
        fieldRef("confirmation_email_recipient_field_name", form.confirmation_email_recipient_field_name, true);
    }
    if (form.send_submission_as_email_enabled) {
        required("submissions_email_id", form.submissions_email_id);
        if (isBlank(form.send_submission_as_email)) {
            required("send_submission_as_email", undefined);
        } else if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(form.send_submission_as_email?.trim() ?? "")) {
            issues.push({ path: "send_submission_as_email", key: "validation.invalidEmail" });
        }
    }
    if (form.activate_cleverreach) {
        required("cleverreach_group_id", form.cleverreach_group_id);
        required("cleverreach_salutation", form.cleverreach_salutation);
        fieldRef("cleverreach_email_field_name", form.cleverreach_email_field_name, true);
        fieldRef("cleverreach_firstname_field_name", form.cleverreach_firstname_field_name, false);
        fieldRef("cleverreach_lastname_field_name", form.cleverreach_lastname_field_name, false);
        if (form.cleverreach_condition.type === "WhenFieldIsSet") {
            fieldRef("cleverreach_condition", form.cleverreach_condition.field, true);
        }
    }
    return issues;
}

export function formatDateTime(date: string | Date, locale: string) {
    return new Date(date).toLocaleString(locale, {
        day: "numeric",
        month: "short",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
    });
}

// "3 hours ago", in the largest unit that fits
export function formatRelative(date: string | Date, locale: string, now = Date.now()) {
    const seconds = Math.round((new Date(date).getTime() - now) / 1000);
    const units: [Intl.RelativeTimeFormatUnit, number][] = [
        ["year", 31536000],
        ["month", 2592000],
        ["week", 604800],
        ["day", 86400],
        ["hour", 3600],
        ["minute", 60],
    ];
    const format = new Intl.RelativeTimeFormat(locale, { numeric: "auto" });
    for (const [unit, size] of units) {
        if (Math.abs(seconds) >= size) return format.format(Math.round(seconds / size), unit);
    }
    return format.format(0, "minute");
}

export type SubmissionEntry = { name: string; label: string; value: string };
// the submitted values in form order with their labels, values of removed fields are appended by name
export function submissionEntries(
    data: unknown,
    form: Form | undefined,
    locale: string,
    booleanLabels: { yes: string; no: string },
): SubmissionEntry[] {
    if (!data || typeof data !== "object" || Array.isArray(data)) return [];
    const values = data as Record<string, unknown>;
    const format = (value: unknown): string => {
        if (value == null || value === "") return "–";
        if (typeof value === "boolean") return value ? booleanLabels.yes : booleanLabels.no;
        if (Array.isArray(value)) return value.map(format).join(", ");
        if (typeof value === "object") return JSON.stringify(value);
        return String(value);
    };
    const fields = form ? dataFields(form.props).filter((f) => f.name in values) : [];
    const known = new Set(fields.map((f) => f.name));
    return [
        ...fields.map((f) => ({ name: f.name, label: f.label[locale] || f.name, value: format(values[f.name]) })),
        ...Object.keys(values)
            .filter((name) => !known.has(name))
            .map((name) => ({ name, label: name, value: format(values[name]) })),
    ];
}
