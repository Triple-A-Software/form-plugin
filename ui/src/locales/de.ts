import type en from "./en";

export default {
    hint: {
        preview:
            "Das ist nur eine grobe Vorschau, das eingebundene Formular auf deiner Seite wird wahrscheinlich etwas unterschiedlich aussehen.",
        cleverreachClientId: "Die Client ID für deinen Cleverreach acccount",
        cleverreachSecret:
            "Dieses Secret wird verwendet, um API-calls zu Cleverreach zu authentifizieren",
        cleverreach_condition:
            "Wann der User zur Cleverreach gruppe hinzugefügt werden soll",
        reCaptchaSecret:
            "Dieses Secret wird verwendet, um API-calls zu reCaptcha zu authentifizieren",
        hCaptchaSecret:
            "Dieses Secret wird verwendet, um API-calls zu hCaptcha zu authentifizieren",
        captchaSitekey: "",
        submissionEmail:
            "Diese E-Mail wird für jede Einsendung an die festgelegte E-Mail-Adresse gesendet",
        submissionEmailRecipient:
            "An diese E-Mail-Adresse wird jede Einsendung gesendet",
    },
    action: {
        goto: {
            forms: "Formulare verwalten",
            emails: "Erreiche deine User",
            submissions: "Siehe Einsendungen",
        },
        create: {
            field: "Feld erstellen",
            option: "Option erstellen",
            form: "Formular erstellen",
            email: "E-mail erstellen",
        },
        save: "Speichern",
        saveAndClose: "Speichern und Schließen",
        no: "Nein",
        yes: "Ja",
        sendEmail: "E-mail senden",
        exportSubmissions: "Einsendungen exportieren",
    },
    modal: {
        edit: {
            field: "Feld bearbeiten: {type}",
            form: "Formular bearbeiten",
            email: "E-mail bearbeiten",
        },
        create: {
            form: "Formular erstellen",
            email: "E-mail erstellen",
        },
        delete: {
            title: "{name} löschen",
            description:
                "Bist du dir sicher dass du diese/s {name} löschen möchtest? Diese Aktion kann nicht rückgängig gemacht werden",
        },
        sendEmail: {
            title: "E-mail versenden",
        },
        exportSubmissions: {
            title: "Einsendungen exportieren",
        },
    },
    deleteName: {
        form: "Formular",
        email: "E-mail",
    },
    field: {
        label: {
            formbuilder: "Form-builder",
            type: "Typ",
            max: "Max",
            maxLength: "Maximale Länge",
            label: "Label",
            hint: "Hinweis",
            class: "Class",
            required: "Benötigt",
            name: "Name",
            placeholder: "Platzhalter",
            min: "Min",
            step: "Schrittgröße",
            value: "Wert",
            data_type: "Datentyp",
            multiple: "Mehrere auswählbar",
            options: "Optionen",
            language: "Sprache",
            sendConfirmationEmail: "Bestätigungs-Email senden",
            confirmationEmail: "Bestätigungs-Email",
            confirmationEmailRecipientFieldName: "Empfänger feld name",
            sendSubmissionAsEmail: "Einreichung per E-mail senden",
            submissionEmail: "Einsendungs-Email",
            submissionEmailRecipient: "Empfängeradresse",
            subject: "Betreff",
            content: "Inhalt",
            color: "Farbe",
            variant: "Variante",
            rows: "Zeilen",
            columns: "Spalten",
            min_label: "Minimum label",
            max_label: "Maximum label",
            preview: "Vorschau",
            default_value: "Standardwert",
            cleverreachClientId: "Cleverreach client ID",
            cleverreachSecret: "Cleverreach secret",
            cleverreachGroupId: "Cleverreach Gruppen-ID",
            activateCleverreach: "Cleverreach Integration aktivieren",
            cleverreach_condition: "Cleverreach Bedingung",
            cleverreach_condition_field: "Feld name",
            cleverreachEmailField: "Cleverreach Email-Feld",
            cleverreachFirstNameField: "Cleverreach Vornamenfeld",
            cleverreachLastNameField: "Cleverreach Nachnamefeld",
            cleverreachSalutation: "Cleverreach Begrüßung",
            emailFieldName: "E-mail feld name",
            captchaType: "Captchatyp",
            activateCaptcha: "Captcha aktivieren",
            reCaptchaSecret: "ReCaptcha secret",
            hCaptchaSecret: "HCaptcha secret",
            captchaSitekey: "Captcha sitekey",
            captchaCallback: "Captcha Callback",
            captchaExpiredCallback: "Captcha abgelaufen Callback",
            onlyExportForm: "Nur Einsendungen für dieses Formular exportieren",
            startDate: "Start Datum",
            endDate: "End Datum",
            noDate: "Kein Datum",
            today: "Heute",
            yesterday: "Gestern",
            oneWeekAgo: "Vor einer Woche",
            oneMonthAgo: "Vor einem Monat",
            input_class: "Input class",
            label_class: "Label class",
            button_class: "Button class",
            min_label_class: "Min label class",
            max_label_class: "Max label class",
        },
        option: {
            type: {
                button: "Button",
                submit: "Absenden",
                reset: "Zurücksetzen",
                password: "Passwort",
                text: "Text",
                email: "E-Mail",
                color: "Farbe",
                tel: "Tel",
                date: "Datum",
                datetimeLocal: "Lokales Datum und Zeit",
                time: "Uhrzeit",
                week: "Woche",
                month: "Monat",
            },
            data_type: {
                string: "String",
                number: "Zahl",
            },
            variant: {
                solid: "Ausgefüllt",
                alternative: "Alternativ",
            },
            default_value: {
                nothing: "Nichts",
            },
            cleverreach_condition: {
                on_submit: "Beim Absenden",
                when_field_is_set: "Wenn ein Feld gesetzt ist",
            },
            captchaType: {
                hCaptcha: "HCaptcha",
                reCaptcha: "ReCaptcha",
            },
        },
        placeholder: {
            confirmationEmail: "Bestätigungs-Email auswählen",
            selectEmail: "E-mail auswählen",
            filterByForm: "Nach Formular filtern",
            selectForm: "Formular auswählen",
        },
        tooltip: {
            sendEmail: "E-mail an diesen User senden",
            exportSubmissions:
                "Alle Einsendungen mit dem aktuellen filter exportieren",
            downloadAttachments: "Dateien von dieser Einsendung herunterladen",
        },
    },
    builder: {
        item: {
            label: {
                text: "Textfeld",
                textarea: "Großes Textfeld",
                markdown: "Markdown",
                richtext: "Formatiertes Textfeld",
                checkbox: "Checkbox",
                number: "Zahl",
                select: "Auswahl",
                code: "Code",
                range: "Zahlen von-bis",
                buttongroup: "Buttongroup",
                button: "Button",
                captcha: "Captcha",
                date: "Datum",
                file: "Datei",
            },
        },
    },
    pages: {
        home: {
            title: "Home",
        },
        forms: {
            title: "Formulare",
            description:
                "Verwalte Formulare, die Besucher deiner Webseite ausfüllen können um dir Informationen zukommen zu lassen oder einen Newsletter zu abonnieren",
        },
        submissions: {
            title: "Einsendungen",
            description:
                "Siehe Einsendungen deiner Formulare, exportiere diese um sie weiter zu analysieren oder sende E-mails an die Absender",
        },
        emails: {
            title: "E-mails",
            description:
                "Verwalte E-mails, die an den Absender eines Formulars oder eine feste E-mailadresse geschickt werden können. So kannst du Besucher benachrichtigen, wenn diese ein Formular auf deiner Webseite ausfüllen",
        },
        settings: {
            title: "Einstellungen",
            cleverreach: "Cleverreach",
        },
    },
    table_columns: {
        name: "Name",
        subject: "Betreff",
        form: "Formular",
        timestamp: "Zeitpunkt",
    },
    color: {
        red: "Rot",
        orange: "Orange",
        amber: "Bernstein",
        yellow: "Gelb",
        lime: "Limette",
        green: "Grün",
        emerald: "Smaragd",
        teal: "Aquamarin",
        cyan: "Cyan",
        sky: "Himmelblau",
        blue: "Blau",
        indigo: "Indigo",
        violet: "Violett",
        purple: "Lila",
        fuchsia: "Fuchsia",
        pink: "Rosa",
        rose: "Rosenfarben",
        slate: "Schiefer",
        gray: "Grau",
        zinc: "Zinc",
        neutral: "Neutral",
        stone: "Stein",
    },
    toast: {
        "/api/emails": {
            GET: {
                error: "Emails konnten nicht geladen werden",
            },
            POST: {
                success: "Email erfolgreich erstellt",
                error: "Email konnte nicht erstellt werden",
            },
            PUT: {
                success: "Email erfolgreich gespeichert",
                error: "Email konnte nicht gespeichert werden",
            },
            DELETE: {
                success: "Email erfolgreich gelöscht",
                error: "Email konnte nicht gelöscht werden",
            },
        },
        "/api/forms": {
            GET: {
                error: "Formulare konnten nicht geladen werden",
            },
            POST: {
                success: "Formular erfolgreich erstellt",
                error: "Formular konnte nicht erstellt werden",
            },
            PUT: {
                success: "Formular erfolgreich gespeichert",
                error: "Formular konnte nicht gespeichert werden",
            },
            DELETE: {
                success: "Formular erfolgreich gelöscht",
                error: "Formular konnte nicht gelöscht werden",
            },
        },
        "/api/settings": {
            GET: {
                error: "Einstellungen konnten nicht geladen werden",
            },
            PUT: {
                success: "Einstellungen erfolgreich gespeichert",
                error: "Einstellungen konnten nicht gespeichert werden",
            },
        },
        "/api/submissions": {
            GET: {
                error: "Einsendungen konnten nicht geladen werden",
            },
        },
        "/api/submissions/send-email": {
            POST: {
                success: "E-mail erfolgreich gesendet",
                error: "E-mail konnte nicht gesendet werden",
            },
        },
    },
} satisfies typeof en;
