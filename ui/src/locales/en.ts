import type { API } from "@/utils/types/api";

export default {
    alert: {
        missingCaptchaSettings: {
            title: "Missing captcha settings",
            description:
                "If you want to use a captcha, you need to configure the corresponding captcha secret and the captcha sitekey in the settings",
        },
    },
    hint: {
        preview: "This is just a rough preview, it is not exactly how the form will be rendered on your page.",
        cleverreachClientId: "This is the client id for your cleverreach account",
        cleverreachSecret: "This secret is used to authenticate api calls to cleverreach",
        cleverreach_condition: "When should the user be added to the cleverreach group?",
        reCaptchaSecret: "This secret will be used to authenticate api calls to reCaptcha",
        hCaptchaSecret: "This secret will be used to authenticate api calls to hCaptcha",
        captchaSitekey: "",
        submissionEmail: "This email will be sent to the specified email address for every submission",
        submissionEmailRecipient: "This email address will receive every submission",
        additionalHtml: "This HTML will be added to the end of form",
        emailContent: "Format: HTML, with {'{{ submission.<field name> }}'} for accessing values of the submission",
    },
    action: {
        goto: {
            forms: "Manage forms",
            emails: "Reach your users",
            submissions: "See submissions",
            settings: "Go to settings",
        },
        create: {
            field: "Create field",
            option: "Create option",
            form: "Create form",
            email: "Create email",
        },
        save: "Save",
        saveAndClose: "Save and close",
        no: "No",
        yes: "Yes",
        sendEmail: "Send email",
        exportSubmissions: "Export submissions",
    },
    modal: {
        edit: {
            field: "Editing field: {type}",
            form: "Editing form",
            email: "Editing email",
        },
        create: {
            form: "Creating form",
            email: "Creating email",
        },
        delete: {
            title: "Delete {name}",
            description: "Are you sure you want to delete this {name}? This action cannot be undone",
        },
        sendEmail: {
            title: "Sending email",
        },
        exportSubmissions: {
            title: "Export submissions",
        },
    },
    deleteName: {
        form: "form",
        email: "email",
    },
    field: {
        label: {
            additionalHtml: "Additional HTML",
            formbuilder: "Form-builder",
            type: "Type",
            max: "Max",
            maxLength: "Max length",
            label: "Label",
            hint: "Hint",
            class: "Class",
            required: "Required",
            name: "Name",
            placeholder: "Placeholder",
            min: "Min",
            step: "Step",
            value: "Value",
            data_type: "Datatype",
            multiple: "Multiple selectable",
            options: "Options",
            language: "Language",
            sendConfirmationEmail: "Send confirmation email",
            confirmationEmail: "Confirmation email",
            confirmationEmailRecipientFieldName: "Recipient field name",
            sendSubmissionAsEmail: "Send submission as email",
            submissionEmail: "Submission email",
            submissionEmailRecipient: "Submission email recipient",
            subject: "Subject",
            content: "Content",
            color: "Color",
            variant: "Variant",
            rows: "Rows",
            columns: "Columns",
            min_label: "Min label",
            max_label: "Max label",
            preview: "Preview",
            default_value: "Default value",
            cleverreachClientId: "Cleverreach client ID",
            cleverreachSecret: "Cleverreach secret",
            cleverreachGroupId: "Cleverreach group ID",
            activateCleverreach: "Activate Cleverreach integration",
            cleverreach_condition: "Cleverreach condition",
            cleverreachEmailField: "Cleverreach email field",
            cleverreachFirstNameField: "Cleverreach first name field",
            cleverreachLastNameField: "Cleverreach last name field",
            cleverreachSalutation: "Cleverreach salutation",
            cleverreach_condition_field: "Field name",
            emailFieldName: "Email field name",
            captchaType: "Captcha type",
            activateCaptcha: "Activate captcha",
            reCaptchaSecret: "ReCaptcha secret",
            hCaptchaSecret: "HCaptcha secret",
            captchaSitekey: "Captcha sitekey",
            captchaCallback: "Captcha callback",
            captchaExpiredCallback: "Captcha expired callback",
            onlyExportForm: "Only export submssions from this form",
            startDate: "Start date",
            endDate: "End date",
            noDate: "No date",
            today: "Today",
            yesterday: "Yesterday",
            oneWeekAgo: "One week ago",
            oneMonthAgo: "One month ago",
            input_class: "Input class",
            label_class: "Label class",
            button_class: "Button class",
            min_label_class: "Min label class",
            max_label_class: "Max label class",
        },
        option: {
            type: {
                button: "Button",
                submit: "Submit",
                reset: "Reset",
                password: "Password",
                text: "Text",
                email: "E-Mail",
                color: "Color",
                tel: "Tel",
                date: "Date",
                datetimeLocal: "Local Datetime",
                time: "Time",
                week: "Week",
                month: "Month",
            },
            data_type: {
                string: "String",
                number: "Number",
            },
            variant: {
                solid: "Solid",
                alternative: "Alternative",
                unstyled: "Unstyled",
            },
            default_value: {
                nothing: "Nothing",
            },
            cleverreach_condition: {
                on_submit: "On submit",
                when_field_is_set: "When field is set",
            },
            captchaType: {
                hCaptcha: "HCaptcha",
                reCaptcha: "ReCaptcha",
                reCaptchaDescription: "Disabled, because ReCaptcha is not supported yet",
            },
        },
        placeholder: {
            confirmationEmail: "Select confirmation email",
            selectEmail: "Select an email",
            filterByForm: "Filter by form",
            selectForm: "Select form",
        },
        tooltip: {
            sendEmail: "Send email to this user",
            exportSubmissions: "Export all submissions matching the current filter",
            downloadAttachments: "Download files from this submission",
        },
    },
    builder: {
        item: {
            label: {
                text: "Textfield",
                textarea: "Textarea",
                markdown: "Markdown",
                richtext: "WYSIWYG",
                checkbox: "Checkbox",
                number: "Number",
                select: "Select",
                code: "Code",
                range: "Range",
                buttongroup: "Buttongroup",
                button: "Button",
                captcha: "Captcha",
                date: "Date",
                file: "File",
            },
        },
    },
    pages: {
        home: {
            title: "Home",
        },
        forms: {
            title: "Forms",
            description: "Manage forms visitors of your website can fill out to send information to you, or sign up to anything",
        },
        submissions: {
            title: "Submissions",
            description:
                "See the submissions on every one of your forms, export them to analyse them further or send emails to the submitters",
        },
        emails: {
            title: "Emails",
            description:
                "Manage emails, which can be sent to form submitters or a fixed email address. This way you can notify visitors when they fill out a form",
        },
        settings: {
            title: "Settings",
            cleverreach: "Cleverreach",
        },
    },
    table_columns: {
        name: "Name",
        subject: "Subject",
        form: "Form",
        timestamp: "Timestamp",
    },
    color: {
        red: "Red",
        orange: "Orange",
        amber: "Amber",
        yellow: "Yellow",
        lime: "Lime",
        green: "Green",
        emerald: "Emerald",
        teal: "Teal",
        cyan: "Cyan",
        sky: "Sky",
        blue: "Blue",
        indigo: "Indigo",
        violet: "Violet",
        purple: "Purple",
        fuchsia: "Fuchsia",
        pink: "Pink",
        rose: "Rose",
        slate: "Slate",
        gray: "Gray",
        zinc: "Zinc",
        neutral: "Neutral",
        stone: "Stone",
    },
    toast: {
        "/api/emails": {
            GET: {
                error: "Failed to fetch emails",
            },
            POST: {
                success: "Email created successfully",
                error: "Failed to create email",
            },
            PUT: {
                success: "Email updated successfully",
                error: "Failed to update email",
            },
            DELETE: {
                success: "Email deleted successfully",
                error: "Failed to delete email",
            },
        },
        "/api/forms": {
            GET: {
                error: "Failed to fetch forms",
            },
            POST: {
                success: "Form created successfully",
                error: "Failed to create form",
            },
            PUT: {
                success: "Form updated successfully",
                error: "Failed to update form",
            },
            DELETE: {
                success: "Form deleted successfully",
                error: "Failed to delete form",
            },
        },
        "/api/settings": {
            GET: {
                error: "Failed to load settings",
            },
            PUT: {
                success: "Settings saved successfully",
                error: "Failed to save settings",
            },
        },
        "/api/submissions": {
            GET: {
                error: "Failed to fetch submissions",
            },
        },
        "/api/submissions/send-email": {
            POST: {
                success: "Email sent successfully",
                error: "Failed to send email",
            },
        },
    } satisfies Partial<{
        [key in keyof API]: {
            [method in keyof API[key]]?: {
                success?: string;
                error?: string;
            };
        };
    }>,
};
