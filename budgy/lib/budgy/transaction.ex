defmodule Budgy.Transaction do
  use Ecto.Schema
  alias Ecto.Changeset

  @primary_key false
  embedded_schema do
    field(:number, :integer)
    field(:statement, :integer)
    field(:name, :string, default: "Jeremy Gillard")
    field(:account, :string)
    field(:counterpart_name, :string)
    field(:counterpart_street, :string)
    field(:counterpart_postal_code, :string)
    field(:counterpart_account, :string)
    field(:counterpart_bic, :string)
    field(:counterpart_country_code, :string)
    field(:value_date, :date)
    field(:posting_date, :date)
    field(:description, :string)
    field(:communication, :string)
    field(:amount, :decimal)
    field(:currency, :string)
  end

  def changeset(filters \\ %__MODULE__{}, params \\ %{}) do
    filters
    |> Changeset.cast(params, __schema__(:fields))
  end

  def from_original(original) do
    %{
      "number" => original["Numéro de transaction"],
      "statement" => original["Numéro d'extrait"],
      "name" => "Jeremy Gillard",
      "account" => original["Compte"],
      "counterpart_name" => original["Nom contrepartie contient"],
      "counterpart_street" => original["Rue et numéro"],
      "counterpart_postal_code" => original["Code postal et localité"],
      "counterpart_account" => original["Compte contrepartie"],
      "counterpart_bic" => original["BIC"],
      "counterpart_country_code" => original["Code pays"],
      "value_date" => original["Date valeur"],
      "posting_date" => original["Date de comptabilisation"],
      "description" => original["Transaction"],
      "communication" => original["Communications"],
      "amount" => original["Montant"],
      "currency" => original["Devise"]
    }
  end

  def to_struct(attrs) do
    attrs =
      attrs
      |> Map.update!("amount", &normalize_decimal/1)
      |> Map.update!("value_date", &normalize_date/1)
      |> Map.update!("posting_date", &normalize_date/1)

    changeset = changeset(%__MODULE__{}, attrs)
    Changeset.apply_changes(changeset)
  end

  def missing_transaction_numbers(transactions) do
    numbers = Enum.map(transactions, & &1.number)
    range = Enum.min(numbers)..Enum.max(numbers)
    MapSet.difference(MapSet.new(range), MapSet.new(numbers)) |> MapSet.to_list()
  end

  defp normalize_decimal(value) when is_binary(value) do
    value
    |> String.replace(",", ".")
  end

  defp normalize_date(value) when is_binary(value) do
    case String.split(value, "/") do
      [day, month, year] ->
        "#{year}-#{month}-#{day}"

      _ ->
        # fallback if format is unexpected
        value
    end
  end
end
