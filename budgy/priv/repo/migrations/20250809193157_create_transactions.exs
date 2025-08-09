defmodule Budgy.Repo.Migrations.CreateTransactions do
  use Ecto.Migration

  def change do
    create table(:transactions, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :number, :integer, null: false
      add :statement_number, :integer, null: false
      add :value_date, :utc_datetime, null: false
      add :posting_date, :utc_datetime, null: false
      add :amount, :decimal, null: false
      add :currency, :string, null: false
      add :description, :string, null: false
      add :communication, :string
      add :senter_id, references(:counterparties, on_delete: :nothing, type: :binary_id), null: false
      add :recipient_id, references(:counterparties, on_delete: :nothing, type: :binary_id), null: false

      timestamps(type: :utc_datetime)
    end

    create index(:transactions, [:senter_id])
    create index(:transactions, [:recipient_id])
  end
end
