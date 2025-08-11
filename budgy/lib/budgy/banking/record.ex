defmodule Budgy.Banking.Record do
  use Ecto.Schema

  alias Ecto.Changeset
  alias Budgy.Banking.Account
  alias Budgy.Banking.Bank
  alias Budgy.Banking.Address

  @primary_key false
  embedded_schema do
    # Transaction
    field(:number, :integer)
    field(:statement_number, :integer)
    field(:value_date, :date)
    field(:posting_date, :date)
    field(:amount, :decimal)
    field(:currency, :string)
    field(:description, :string)
    field(:communication, :string)
    # User
    field(:name, :string)
    field(:account, :string)
    field(:bic, :string)
    field(:country_code, :string)
    field(:street, :string)
    field(:street_number, :string)
    field(:postal_code, :string)
    field(:city, :string)
    # Counterparty
    field(:counterpart_name, :string)
    field(:counterpart_account, :string)
    field(:counterpart_bic, :string)
    field(:counterpart_country_code, :string)
    field(:counterpart_street, :string)
    field(:counterpart_street_number, :string)
    field(:counterpart_postal_code, :string)
    field(:counterpart_cit, :string)
  end

  def changeset(schema, attrs) do
    schema
    |> Changeset.cast(attrs, __schema__(:fields))
    |> Changeset.validate_required([
      :number,
      :statement_number,
      :value_date,
      :posting_date,
      :amount,
      :currency,
      :description,
      # :communication,
      # User
      :name,
      :account,
      :bic,
      :country_code,
      :street,
      :street_number,
      :postal_code,
      :city,
      # Counterparty
      # :counterpart_name,
      :counterpart_account
      # :counterpart_bic,
      # :counterpart_country_code,
      # :counterpart_street,
      # :counterpart_street_number,
      # :counterpart_postal_code,
      # :counterpart_city
    ])
  end

  def from_row(
        row,
        username \\ "",
        %Account{} = account \\ %Account{},
        %Bank{} = bank \\ %Bank{},
        %Address{} = address \\ %Address{}
      ) do
    %{
      "number" => row["Numéro de transaction"],
      "statement_number" => row["Numéro d'extrait"],
      "value_date" => normalize_date(row["Date valeur"]),
      "posting_date" => normalize_date(row["Date de comptabilisation"]),
      "amount" => String.replace(row["Montant"], ",", "."),
      "currency" => row["Devise"],
      "description" => row["Transaction"],
      "communication" => row["Communications"],
      # User
      "name" => username || row[""],
      "account" => account.number || row["Compte"],
      "bic" => bank.bic || row[""],
      "country_code" => bank.country_code || row[""],
      "street" => address.street || row[""],
      "street_number" => address.number || row[""],
      "postal_code" => address.postal_code || row[""],
      "city" => address.city || row[""],
      # Counterparty
      "counterpart_name" => row["Nom contrepartie contient"],
      "counterpart_account" => row["Compte contrepartie"],
      "counterpart_bic" => row["BIC"],
      "counterpart_country_code" => row["Code pays"],
      "counterpart_street" => String.split(row["Rue et numéro"], " ") |> List.first(),
      "counterpart_street_number" => String.split(row["Rue et numéro"], " ") |> List.first(),
      "counterpart_postal_code" =>
        String.split(row["Code postal et localité"], " ") |> List.first(),
      "counterpart_city" => String.split(row["Code postal et localité"], " ") |> List.first()
    }
  end

  def struct!(attrs) do
    %__MODULE__{}
    |> changeset(attrs)
    |> Changeset.apply_changes()
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
