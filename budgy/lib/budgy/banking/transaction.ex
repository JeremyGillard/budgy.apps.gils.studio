defmodule Budgy.Banking.Transaction do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "transactions" do
    field :number, :integer
    field :statement_number, :integer
    field :value_date, :utc_datetime
    field :posting_date, :utc_datetime
    field :amount, :decimal
    field :currency, :string
    field :description, :string
    field :communication, :string
    field :senter_id, :binary_id
    field :recipient_id, :binary_id

    timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(transaction, attrs) do
    transaction
    |> cast(attrs, [:number, :statement_number, :value_date, :posting_date, :amount, :currency, :description, :communication])
    |> validate_required([:number, :statement_number, :value_date, :posting_date, :amount, :currency, :description, :communication])
  end
end
