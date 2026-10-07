import { useState } from "react";
import { ChevronDown, Search } from "lucide-react";
import { PageHeader } from "@/components/layout/PageHeader";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { t } from "@/i18n";

/** Lowercase, without accents: "Créer" matches "creer". */
function normalize(text: string) {
  return text
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();
}

/** FAQ, by category, with a search over questions and answers. */
export function HelpPage() {
  const [search, setSearch] = useState("");
  const query = normalize(search.trim());

  const categories = t.faq.categories
    .map((category) => ({
      title: category.title,
      items: category.items.filter(
        (item) => !query || normalize(`${item.q} ${item.a}`).includes(query),
      ),
    }))
    .filter((category) => category.items.length > 0);

  return (
    <div className="space-y-6">
      <PageHeader title={t.faq.title} description={t.faq.description} />

      <div className="relative max-w-md">
        <Search className="absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder={t.faq.search}
          aria-label={t.faq.search}
          className="pl-9"
        />
      </div>

      {categories.length === 0 && (
        <p className="text-sm text-muted-foreground">{t.faq.noResult}</p>
      )}

      {categories.map((category) => (
        <Card key={category.title}>
          <CardHeader>
            <CardTitle>{category.title}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {category.items.map((item) => (
              // Native disclosure: accessible, no extra dependency.
              <details
                key={item.q}
                // Searching opens the matching answers.
                open={query ? true : undefined}
                className="group rounded-lg border px-3 open:bg-muted/40"
              >
                <summary className="flex cursor-pointer list-none items-center gap-2 py-3 text-sm font-medium outline-none focus-visible:ring-3 focus-visible:ring-ring/50 [&::-webkit-details-marker]:hidden">
                  <span className="flex-1">{item.q}</span>
                  <ChevronDown className="size-4 shrink-0 text-muted-foreground transition-transform duration-200 group-open:rotate-180" />
                </summary>
                <p
                  data-selectable
                  className="pb-3 text-sm whitespace-pre-line text-muted-foreground"
                >
                  {item.a}
                </p>
              </details>
            ))}
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
