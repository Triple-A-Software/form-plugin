import { expect, test } from "bun:test";
import type { Form } from "../../bindings";
import { createElement, createText, formatRelative, slugify, submissionEntries, validateForm } from "./utils";

const createCaptcha = () => createElement("captcha")!;

function form(overrides: Partial<Form> = {}): Form {
    return {
        id: -1,
        name: "Contact",
        send_confirmation_email: false,
        confirmation_email_id: undefined,
        confirmation_email_recipient_field_name: undefined,
        send_submission_as_email_enabled: false,
        send_submission_as_email: undefined,
        submissions_email_id: undefined,
        email_attach_files: true,
        activate_cleverreach: false,
        cleverreach_condition: { type: "OnSubmit" },
        cleverreach_group_id: undefined,
        cleverreach_email_field_name: undefined,
        cleverreach_firstname_field_name: undefined,
        cleverreach_lastname_field_name: undefined,
        cleverreach_salutation: undefined,
        activate_captcha: false,
        captcha_callback: undefined,
        captcha_expired_callback: undefined,
        props: [],
        additional_html: "",
        ...overrides,
    };
}
const field = (name: string) => ({ ...createText(), name });
const paths = (f: Form) => validateForm(f).map((i) => `${i.path}:${i.key}`);

test("valid form has no issues", () => {
    expect(paths(form({ props: [field("email")] }))).toEqual([]);
});

test("missing form name and field names", () => {
    expect(paths(form({ name: " ", props: [field("")] }))).toEqual(["name:validation.required", "props.0:validation.fieldName"]);
});

test("duplicate field names and multiple captchas", () => {
    const props = [field("a"), field("a"), createCaptcha(), createCaptcha()];
    expect(paths(form({ props, activate_captcha: true }))).toEqual([
        "props.1:validation.duplicateName",
        "props:validation.multipleCaptchas",
    ]);
});

test("captcha element and verification must be enabled together", () => {
    expect(paths(form({ activate_captcha: true }))).toEqual(["activate_captcha:validation.captchaMissing"]);
    expect(paths(form({ props: [createCaptcha()] }))).toEqual(["activate_captcha:validation.captchaUnchecked"]);
});

test("references must point to existing fields", () => {
    const f = form({
        props: [field("email")],
        send_confirmation_email: true,
        confirmation_email_id: 1,
        confirmation_email_recipient_field_name: "mail",
    });
    expect(paths(f)).toEqual(["confirmation_email_recipient_field_name:validation.unknownField"]);
});

test("submission email needs a valid recipient", () => {
    const f = form({ send_submission_as_email_enabled: true, submissions_email_id: 1, send_submission_as_email: "nope" });
    expect(paths(f)).toEqual(["send_submission_as_email:validation.invalidEmail"]);
});

test("cleverreach requires group, salutation and an email field, names are optional", () => {
    const f = form({
        props: [field("email")],
        activate_cleverreach: true,
        cleverreach_email_field_name: "email",
        cleverreach_condition: { type: "WhenFieldIsSet", field: "newsletter" },
    });
    expect(paths(f)).toEqual([
        "cleverreach_group_id:validation.required",
        "cleverreach_salutation:validation.required",
        "cleverreach_condition:validation.unknownField",
    ]);
});

test("slugify makes field names from labels", () => {
    expect(slugify("E-Mail Adresse")).toBe("e_mail_adresse");
    expect(slugify(" Größe (cm) ")).toBe("grosse_cm");
});

test("formatRelative picks the largest fitting unit", () => {
    const now = new Date("2026-01-10T12:00:00Z").getTime();
    expect(formatRelative("2026-01-10T11:59:30Z", "en", now)).toBe("this minute");
    expect(formatRelative("2026-01-10T09:00:00Z", "en", now)).toBe("3 hours ago");
    expect(formatRelative("2026-01-09T12:00:00Z", "en", now)).toBe("yesterday");
});

test("submissionEntries uses labels, form order and readable values", () => {
    const f = form({ props: [{ ...field("b"), label: { en: "Bee" } }, field("a")] });
    const entries = submissionEntries({ a: ["x", "y"], b: true, old: null }, f, "en", { yes: "Yes", no: "No" });
    expect(entries).toEqual([
        { name: "b", label: "Bee", value: "Yes" },
        { name: "a", label: "a", value: "x, y" },
        { name: "old", label: "old", value: "–" },
    ]);
});
