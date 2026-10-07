import { useState } from "react";
import { RotateCcw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { t } from "@/i18n";
import { choiceLabel, formatValue, settingText } from "@/lib/gameSettings";
import type { SettingDef, SettingValue } from "@/types";

/** One setting: label and description on the left, the right control on the right. */
export function SettingField({
  def,
  value,
  onChange,
}: {
  def: SettingDef;
  value: SettingValue;
  onChange: (value: SettingValue) => void;
}) {
  const { label, description } = settingText(def.key);
  const id = `setting-${def.key}`;
  const changed = value !== def.default;

  return (
    <div className="flex items-center gap-4 py-3">
      <div className="min-w-0 flex-1 space-y-0.5">
        <Label htmlFor={id}>{label}</Label>
        {description && (
          <p className="text-xs text-muted-foreground">{description}</p>
        )}
        {changed && (
          <p className="text-xs text-primary">
            {t.gameSettings.defaultValue(formatValue(def, def.default))}
          </p>
        )}
      </div>
      {changed && (
        <Button
          type="button"
          variant="ghost"
          size="icon-sm"
          onClick={() => onChange(def.default)}
          aria-label={t.gameSettings.reset}
          title={t.gameSettings.reset}
        >
          <RotateCcw />
        </Button>
      )}
      <Control id={id} def={def} value={value} onChange={onChange} />
    </div>
  );
}

function Control({
  id,
  def,
  value,
  onChange,
}: {
  id: string;
  def: SettingDef;
  value: SettingValue;
  onChange: (value: SettingValue) => void;
}) {
  switch (def.kind) {
    case "bool":
      return (
        <Switch id={id} checked={value === true} onCheckedChange={onChange} />
      );
    case "int":
      return (
        <IntInput
          id={id}
          value={Number(value)}
          min={def.min}
          max={def.max}
          onChange={onChange}
        />
      );
    case "choice":
      return (
        <Select value={String(value)} onValueChange={onChange}>
          <SelectTrigger id={id} className="w-36">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {def.choices.map((choice) => (
              <SelectItem key={choice} value={choice}>
                {choiceLabel(def.key, choice)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    case "text":
      return (
        <Input
          id={id}
          value={String(value)}
          maxLength={def.maxLength}
          onChange={(e) => onChange(e.target.value)}
          className="w-56"
        />
      );
  }
}

/**
 * Number field that only reports valid values: while the text is incomplete
 * or out of bounds, the last valid value is kept (and restored on blur).
 */
function IntInput({
  id,
  value,
  min,
  max,
  onChange,
}: {
  id: string;
  value: number;
  min: number;
  max: number | null;
  onChange: (value: number) => void;
}) {
  const [text, setText] = useState(String(value));
  // Follow outside changes (reset button, discard).
  const [shown, setShown] = useState(value);
  if (shown !== value) {
    setShown(value);
    setText(String(value));
  }

  const isValid = (raw: string) => {
    const n = Number(raw);
    return (
      raw.trim() !== "" &&
      Number.isInteger(n) &&
      n >= min &&
      (max === null || n <= max)
    );
  };

  return (
    <Input
      id={id}
      type="number"
      inputMode="numeric"
      min={min}
      max={max ?? undefined}
      value={text}
      aria-invalid={!isValid(text)}
      onChange={(e) => {
        setText(e.target.value);
        if (isValid(e.target.value)) {
          setShown(Number(e.target.value));
          onChange(Number(e.target.value));
        }
      }}
      onBlur={() => setText(String(value))}
      className="w-28"
    />
  );
}
