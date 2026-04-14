import 'package:flutter/material.dart';
import 'package:ourchat/core/chore.dart';
import 'package:ourchat/l10n/app_localizations.dart';
import 'package:ourchat/main.dart';
import 'package:ourchat/core/account.dart';
import 'package:provider/provider.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;

/// Pantalla para gestionar cuentas externas (DeltaChat y ArcaneChat)
class ExternalChatManagement extends StatefulWidget {
  const ExternalChatManagement({super.key});

  @override
  State<ExternalChatManagement> createState() => _ExternalChatManagementState();
}

class _ExternalChatManagementState extends State<ExternalChatManagement> {
  bool _isLoading = false;
  List<Map<String, dynamic>> _externalAccounts = [];

  @override
  void initState() {
    super.initState();
    _loadExternalAccounts();
  }

  Future<void> _loadExternalAccounts() async {
    setState(() => _isLoading = true);
    try {
      final ourchatAppState = context.read<OurChatAppState>();
      final account = ourchatAppState.thisAccount;
      if (account == null) return;

      // TODO: Implementar llamada HTTP al backend para obtener cuentas externas
      // Por ahora, lista vacía como placeholder
      setState(() {
        _externalAccounts = [];
      });
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Error al cargar cuentas: $e')),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _isLoading = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    var l10n = AppLocalizations.of(context)!;
    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.externalChats),
        actions: [
          IconButton(
            icon: const Icon(Icons.add),
            onPressed: () => _showCreateAccountDialog(context),
            tooltip: l10n.createExternalAccount,
          ),
        ],
      ),
      body: _isLoading
          ? const Center(child: CircularProgressIndicator())
          : _externalAccounts.isEmpty
              ? Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(
                        Icons.chat_outlined,
                        size: 64,
                        color: Colors.grey[400],
                      ),
                      const SizedBox(height: 16),
                      Text(
                        l10n.noExternalAccounts,
                        style: TextStyle(
                          fontSize: 16,
                          color: Colors.grey[600],
                        ),
                      ),
                      const SizedBox(height: 24),
                      ElevatedButton.icon(
                        icon: const Icon(Icons.add),
                        label: Text(l10n.createExternalAccount),
                        onPressed: () => _showCreateAccountDialog(context),
                      ),
                    ],
                  ),
                )
              : ListView.builder(
                  itemCount: _externalAccounts.length,
                  itemBuilder: (context, index) {
                    final account = _externalAccounts[index];
                    return ListTile(
                      leading: CircleAvatar(
                        backgroundColor: account['type'] == 'deltachat'
                            ? Colors.blue
                            : Colors.purple,
                        child: Icon(
                          account['type'] == 'deltachat'
                              ? Icons.email
                              : Icons.lock_outline,
                          color: Colors.white,
                        ),
                      ),
                      title: Text(account['display_name'] ?? account['address']),
                      subtitle: Text(account['address']),
                      trailing: PopupMenuButton<String>(
                        onSelected: (value) => _handleAccountAction(value, account),
                        itemBuilder: (context) => [
                          const PopupMenuItem(
                            value: 'switch',
                            child: ListTile(
                              leading: Icon(Icons.swap_horiz),
                              title: Text('Cambiar a esta cuenta'),
                            ),
                          ),
                          const PopupMenuItem(
                            value: 'remove',
                            child: ListTile(
                              leading: Icon(Icons.delete, color: Colors.red),
                              title: Text('Eliminar cuenta', style: TextStyle(color: Colors.red)),
                            ),
                          ),
                        ],
                      ),
                    );
                  },
                ),
    );
  }

  void _showCreateAccountDialog(BuildContext context) {
    var l10n = AppLocalizations.of(context)!;
    showDialog(
      context: context,
      builder: (context) => CreateExternalAccountDialog(
        onAccountCreated: () {
          _loadExternalAccounts();
          Navigator.pop(context);
        },
      ),
    );
  }

  void _handleAccountAction(String action, Map<String, dynamic> account) {
    switch (action) {
      case 'switch':
        // TODO: Implementar cambio de cuenta
        break;
      case 'remove':
        // TODO: Implementar eliminación de cuenta
        break;
    }
  }
}

/// Diálogo para crear una cuenta externa (DeltaChat o ArcaneChat)
class CreateExternalAccountDialog extends StatefulWidget {
  final VoidCallback? onAccountCreated;

  const CreateExternalAccountDialog({super.key, this.onAccountCreated});

  @override
  State<CreateExternalAccountDialog> createState() =>
      _CreateExternalAccountDialogState();
}

class _CreateExternalAccountDialogState
    extends State<CreateExternalAccountDialog> {
  String _selectedType = 'deltachat'; // 'deltachat' o 'arcanechat'
  String _chatmailServer = 'https://arcanechat.me';
  String _username = '';
  String _displayName = '';
  String _email = '';
  String _password = '';
  bool _isCreating = false;
  bool _isLinkingExisting = false;

  final _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    var l10n = AppLocalizations.of(context)!;
    return Dialog(
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              Text(
                _isLinkingExisting
                    ? l10n.linkExternalAccount
                    : l10n.createExternalAccount,
                style: Theme.of(context).textTheme.headlineSmall,
              ),
              const SizedBox(height: 16),
              
              // Selector de tipo de cuenta
              DropdownButtonFormField<String>(
                decoration: InputDecoration(
                  labelText: l10n.accountType,
                  border: const OutlineInputBorder(),
                ),
                value: _selectedType,
                items: [
                  DropdownMenuItem(
                    value: 'deltachat',
                    child: Row(
                      children: [
                        Icon(Icons.email, color: Colors.blue),
                        const SizedBox(width: 8),
                        const Text('DeltaChat'),
                      ],
                    ),
                  ),
                  DropdownMenuItem(
                    value: 'arcanechat',
                    child: Row(
                      children: [
                        Icon(Icons.lock_outline, color: Colors.purple),
                        const SizedBox(width: 8),
                        const Text('ArcaneChat'),
                      ],
                    ),
                  ),
                ],
                onChanged: (value) {
                  setState(() {
                    _selectedType = value!;
                    if (_selectedType == 'arcanechat') {
                      _chatmailServer = 'https://arcanechat.me';
                    } else {
                      _chatmailServer = '';
                    }
                  });
                },
              ),
              const SizedBox(height: 16),

              // Toggle entre crear nueva o vincular existente
              SwitchListTile(
                title: Text(_isLinkingExisting
                    ? l10n.linkExistingAccount
                    : l10n.createNewAccount),
                value: _isLinkingExisting,
                onChanged: (value) {
                  setState(() {
                    _isLinkingExisting = value;
                  });
                },
              ),
              const SizedBox(height: 16),

              if (!_isLinkingExisting) ...[
                // Servidor Chatmail (para creación)
                TextFormField(
                  decoration: InputDecoration(
                    labelText: l10n.chatmailServer,
                    hintText: 'https://arcanechat.me',
                    border: const OutlineInputBorder(),
                  ),
                  initialValue: _chatmailServer,
                  keyboardType: TextInputType.url,
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.serverRequired;
                    }
                    if (!value.startsWith('http')) {
                      return l10n.invalidUrl;
                    }
                    return null;
                  },
                  onSaved: (value) => _chatmailServer = value!,
                ),
                const SizedBox(height: 16),

                // Nombre de usuario
                TextFormField(
                  decoration: InputDecoration(
                    labelText: l10n.username,
                    border: const OutlineInputBorder(),
                  ),
                  textInputAction: TextInputAction.next,
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.usernameRequired;
                    }
                    if (value.length < 3) {
                      return l10n.usernameTooShort;
                    }
                    return null;
                  },
                  onSaved: (value) => _username = value!,
                ),
              ] else ...[
                // Email/Address (para vincular)
                TextFormField(
                  decoration: InputDecoration(
                    labelText: _selectedType == 'deltachat'
                        ? l10n.email
                        : l10n.chatmailAddress,
                    border: const OutlineInputBorder(),
                  ),
                  keyboardType: _selectedType == 'deltachat'
                      ? TextInputType.emailAddress
                      : TextInputType.text,
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.addressRequired;
                    }
                    return null;
                  },
                  onSaved: (value) => _email = value!,
                ),
                const SizedBox(height: 16),

                // Contraseña/Token
                TextFormField(
                  decoration: InputDecoration(
                    labelText: l10n.password,
                    border: const OutlineInputBorder(),
                  ),
                  obscureText: true,
                  validator: (value) {
                    if (value == null || value.isEmpty) {
                      return l10n.passwordRequired;
                    }
                    return null;
                  },
                  onSaved: (value) => _password = value!,
                ),
              ],
              const SizedBox(height: 16),

              // Nombre para mostrar
              TextFormField(
                decoration: InputDecoration(
                  labelText: l10n.displayName,
                  border: const OutlineInputBorder(),
                ),
                textInputAction: TextInputAction.done,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return l10n.displayNameRequired;
                  }
                  return null;
                },
                onSaved: (value) => _displayName = value!,
              ),
              const SizedBox(height: 24),

              // Botones de acción
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  TextButton(
                    onPressed: _isCreating ? null : () => Navigator.pop(context),
                    child: Text(l10n.cancel),
                  ),
                  const SizedBox(width: 16),
                  ElevatedButton(
                    onPressed: _isCreating ? null : _submitForm,
                    child: _isCreating
                        ? const SizedBox(
                            width: 20,
                            height: 20,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          )
                        : Text(_isLinkingExisting
                            ? l10n.link
                            : l10n.create),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _submitForm() async {
    if (!_formKey.currentState!.validate()) return;

    _formKey.currentState!.save();

    setState(() => _isCreating = true);

    try {
      final ourchatAppState = context.read<OurChatAppState>();
      final account = ourchatAppState.thisAccount;
      
      if (account == null) {
        throw Exception('No hay una sesión activa');
      }

      // Construir URL del endpoint según el tipo de operación
      String endpoint;
      Map<String, dynamic> payload;

      if (!_isLinkingExisting) {
        // Crear nueva cuenta
        endpoint = _selectedType == 'deltachat'
            ? '/api/external/deltachat/create'
            : '/api/external/arcanechat/create';
        
        payload = {
          'chatmail_server': _chatmailServer,
          'username': _username,
          'display_name': _displayName,
        };
      } else {
        // Vincular cuenta existente
        endpoint = '/api/external/link';
        payload = {
          'account_type': _selectedType,
          'address': _email,
          'credentials': _password,
          'display_name': _displayName,
        };
      }

      // Obtener token de autenticación
      final token = await account.getToken();
      
      // Hacer petición HTTP al backend
      final response = await http.post(
        Uri.parse('${ourchatAppState.server.host}:${ourchatAppState.server.port}$endpoint'),
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer $token',
        },
        body: jsonEncode(payload),
      );

      if (response.statusCode == 200 || response.statusCode == 201) {
        if (mounted) {
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(
              content: Text(_isLinkingExisting
                  ? l10n.accountLinkedSuccessfully
                  : l10n.accountCreatedSuccessfully),
              backgroundColor: Colors.green,
            ),
          );
          widget.onAccountCreated?.call();
        }
      } else {
        final errorData = jsonDecode(response.body);
        throw Exception(errorData['message'] ?? 'Error al crear cuenta');
      }
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text('Error: ${e.toString()}'),
            backgroundColor: Colors.red,
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() => _isCreating = false);
      }
    }
  }
}

/// Widget de avatar para cuentas externas
class ExternalAccountAvatar extends StatelessWidget {
  final String accountType;
  final String? imageUrl;
  final double size;

  const ExternalAccountAvatar({
    super.key,
    required this.accountType,
    this.imageUrl,
    this.size = 40,
  });

  @override
  Widget build(BuildContext context) {
    if (imageUrl != null && imageUrl!.isNotEmpty) {
      return CircleAvatar(
        radius: size / 2,
        backgroundImage: NetworkImage(imageUrl!),
      );
    }

    return CircleAvatar(
      radius: size / 2,
      backgroundColor: accountType == 'deltachat' ? Colors.blue : Colors.purple,
      child: Icon(
        accountType == 'deltachat' ? Icons.email : Icons.lock_outline,
        color: Colors.white,
        size: size * 0.6,
      ),
    );
  }
}
