defmodule BudgyWeb.BankLive.Form do
  use BudgyWeb, :live_view

  alias Budgy.Banking
  alias Budgy.Banking.Bank

  @impl true
  def render(assigns) do
    ~H"""
    <Layouts.app flash={@flash}>
      <.header>
        {@page_title}
        <:subtitle>Use this form to manage bank records in your database.</:subtitle>
      </.header>

      <.form for={@form} id="bank-form" phx-change="validate" phx-submit="save">
        <.input field={@form[:name]} type="text" label="Name" />
        <.input field={@form[:bic]} type="text" label="Bic" />
        <.input field={@form[:country_code]} type="text" label="Country code" />
        <footer>
          <.button phx-disable-with="Saving..." variant="primary">Save Bank</.button>
          <.button navigate={return_path(@return_to, @bank)}>Cancel</.button>
        </footer>
      </.form>
    </Layouts.app>
    """
  end

  @impl true
  def mount(params, _session, socket) do
    {:ok,
     socket
     |> assign(:return_to, return_to(params["return_to"]))
     |> apply_action(socket.assigns.live_action, params)}
  end

  defp return_to("show"), do: "show"
  defp return_to(_), do: "index"

  defp apply_action(socket, :edit, %{"id" => id}) do
    bank = Banking.get_bank!(id)

    socket
    |> assign(:page_title, "Edit Bank")
    |> assign(:bank, bank)
    |> assign(:form, to_form(Banking.change_bank(bank)))
  end

  defp apply_action(socket, :new, _params) do
    bank = %Bank{}

    socket
    |> assign(:page_title, "New Bank")
    |> assign(:bank, bank)
    |> assign(:form, to_form(Banking.change_bank(bank)))
  end

  @impl true
  def handle_event("validate", %{"bank" => bank_params}, socket) do
    changeset = Banking.change_bank(socket.assigns.bank, bank_params)
    {:noreply, assign(socket, form: to_form(changeset, action: :validate))}
  end

  def handle_event("save", %{"bank" => bank_params}, socket) do
    save_bank(socket, socket.assigns.live_action, bank_params)
  end

  defp save_bank(socket, :edit, bank_params) do
    case Banking.update_bank(socket.assigns.bank, bank_params) do
      {:ok, bank} ->
        {:noreply,
         socket
         |> put_flash(:info, "Bank updated successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, bank))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp save_bank(socket, :new, bank_params) do
    case Banking.create_bank(bank_params) do
      {:ok, bank} ->
        {:noreply,
         socket
         |> put_flash(:info, "Bank created successfully")
         |> push_navigate(to: return_path(socket.assigns.return_to, bank))}

      {:error, %Ecto.Changeset{} = changeset} ->
        {:noreply, assign(socket, form: to_form(changeset))}
    end
  end

  defp return_path("index", _bank), do: ~p"/banks"
  defp return_path("show", bank), do: ~p"/banks/#{bank}"
end
