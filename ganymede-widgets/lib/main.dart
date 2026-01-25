// Ganymede Widgets (Flutter)
import 'package:flutter/material.dart';

void main() {
  runApp(const GanymedeDashboard());
}

class GanymedeDashboard extends StatelessWidget {
  const GanymedeDashboard({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Ganymede Dashboard',
      home: Scaffold(
        backgroundColor: const Color(0xFF181A2A),
        body: Center(
          child: Opacity(
            opacity: 0.8,
            child: Container(
              width: 600,
              height: 400,
              decoration: BoxDecoration(
                color: const Color(0xFF23244D),
                borderRadius: BorderRadius.circular(32),
                boxShadow: [
                  BoxShadow(
                    color: Colors.pinkAccent.withOpacity(0.3),
                    blurRadius: 24,
                    spreadRadius: 2,
                  ),
                ],
              ),
              child: const Center(
                child: Text(
                  'Dashboard Ganymede (MVP)',
                  style: TextStyle(
                    color: Colors.white,
                    fontSize: 28,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
