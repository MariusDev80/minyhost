import { en } from "./en";
import { fr } from "./fr";

/** Turns the literal types of `fr` (`as const`) into a shape other locales can fill. */
type Widen<T> = T extends string
  ? string
  : T extends (...args: infer A) => infer R
    ? (...args: A) => Widen<R>
    : { readonly [K in keyof T]: Widen<T[K]> };

/** Every locale has exactly the keys of the French reference file. */
export type Messages = Widen<typeof fr>;

// To add a language: create `<code>.ts` typed as `Messages`, then register it here.
const messages = { fr, en } satisfies Record<string, Messages>;

export type Locale = keyof typeof messages;

/** Each language is shown in its own name, whatever the current language. */
export const localeNames: Record<Locale, string> = {
  fr: "Français",
  en: "English",
};

export const locales = Object.keys(messages) as Locale[];

/** Locale of the OS (through the webview), or English if it is not supported. */
export function systemLocale(): Locale {
  for (const tag of navigator.languages) {
    const code = tag.split("-")[0].toLowerCase();
    if (code in messages) return code as Locale;
  }
  return "en";
}

/**
 * Texts of the current locale. A live binding: modules read `t.x` at call
 * time, so they see the new texts once `setLocale` ran and the tree re-rendered.
 */
export let t: Messages = fr;

export function setLocale(locale: Locale) {
  t = messages[locale];
}
