defmodule BudgyWeb.TransactionLiveTest do
  use BudgyWeb.ConnCase

  import Phoenix.LiveViewTest
  import Budgy.BankingFixtures

  @create_attrs %{description: "some description", number: 42, currency: "some currency", statement_number: 42, value_date: "2025-08-08T19:31:00Z", posting_date: "2025-08-08T19:31:00Z", amount: "120.5", communication: "some communication"}
  @update_attrs %{description: "some updated description", number: 43, currency: "some updated currency", statement_number: 43, value_date: "2025-08-09T19:31:00Z", posting_date: "2025-08-09T19:31:00Z", amount: "456.7", communication: "some updated communication"}
  @invalid_attrs %{description: nil, number: nil, currency: nil, statement_number: nil, value_date: nil, posting_date: nil, amount: nil, communication: nil}
  defp create_transaction(_) do
    transaction = transaction_fixture()

    %{transaction: transaction}
  end

  describe "Index" do
    setup [:create_transaction]

    test "lists all transactions", %{conn: conn, transaction: transaction} do
      {:ok, _index_live, html} = live(conn, ~p"/transactions")

      assert html =~ "Listing Transactions"
      assert html =~ transaction.currency
    end

    test "saves new transaction", %{conn: conn} do
      {:ok, index_live, _html} = live(conn, ~p"/transactions")

      assert {:ok, form_live, _} =
               index_live
               |> element("a", "New Transaction")
               |> render_click()
               |> follow_redirect(conn, ~p"/transactions/new")

      assert render(form_live) =~ "New Transaction"

      assert form_live
             |> form("#transaction-form", transaction: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#transaction-form", transaction: @create_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/transactions")

      html = render(index_live)
      assert html =~ "Transaction created successfully"
      assert html =~ "some currency"
    end

    test "updates transaction in listing", %{conn: conn, transaction: transaction} do
      {:ok, index_live, _html} = live(conn, ~p"/transactions")

      assert {:ok, form_live, _html} =
               index_live
               |> element("#transactions-#{transaction.id} a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/transactions/#{transaction}/edit")

      assert render(form_live) =~ "Edit Transaction"

      assert form_live
             |> form("#transaction-form", transaction: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, index_live, _html} =
               form_live
               |> form("#transaction-form", transaction: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/transactions")

      html = render(index_live)
      assert html =~ "Transaction updated successfully"
      assert html =~ "some updated currency"
    end

    test "deletes transaction in listing", %{conn: conn, transaction: transaction} do
      {:ok, index_live, _html} = live(conn, ~p"/transactions")

      assert index_live |> element("#transactions-#{transaction.id} a", "Delete") |> render_click()
      refute has_element?(index_live, "#transactions-#{transaction.id}")
    end
  end

  describe "Show" do
    setup [:create_transaction]

    test "displays transaction", %{conn: conn, transaction: transaction} do
      {:ok, _show_live, html} = live(conn, ~p"/transactions/#{transaction}")

      assert html =~ "Show Transaction"
      assert html =~ transaction.currency
    end

    test "updates transaction and returns to show", %{conn: conn, transaction: transaction} do
      {:ok, show_live, _html} = live(conn, ~p"/transactions/#{transaction}")

      assert {:ok, form_live, _} =
               show_live
               |> element("a", "Edit")
               |> render_click()
               |> follow_redirect(conn, ~p"/transactions/#{transaction}/edit?return_to=show")

      assert render(form_live) =~ "Edit Transaction"

      assert form_live
             |> form("#transaction-form", transaction: @invalid_attrs)
             |> render_change() =~ "can&#39;t be blank"

      assert {:ok, show_live, _html} =
               form_live
               |> form("#transaction-form", transaction: @update_attrs)
               |> render_submit()
               |> follow_redirect(conn, ~p"/transactions/#{transaction}")

      html = render(show_live)
      assert html =~ "Transaction updated successfully"
      assert html =~ "some updated currency"
    end
  end
end
