defmodule Budgy.Banking.Bank do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "banks" do
    field :name, :string
    field :bic, :string
    field :country_code, :string

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(bank, attrs) do
    bank
    |> cast(attrs, [:name, :bic, :country_code])
    |> validate_required([:name, :bic, :country_code])
  end
end
